use std::{env, fs::File, io::{BufReader, BufWriter}, mem::transmute, net::TcpStream, path::PathBuf, process::exit, thread, time::Duration};

use anyhow::anyhow;
use bevy_ecs::{schedule::Schedule, world::World};
use clap::Parser;
use lumberjack::{offsets::Offsets, types::Types, Dump};
use once_cell::unsync::Lazy;
use parking_lot::{Mutex, RwLock};
use pdb::{FallibleIterator, SymbolData, PDB};
use retour::{Function, GenericDetour};
use rustc_demangle::demangle;
use tracing::{debug, error, info, Level};
#[cfg(windows)]
use windows::{core::s, Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleA}};

static SCHEDULE_RUN_DETOUR: RwLock<Option<GenericDetour<extern "cdecl" fn(*mut Schedule, *mut World)>>> = RwLock::new(None);
static TYPES: Mutex<Option<Types>> = Mutex::new(None);

const SCHEDULE_RUN: &str = "bevy_ecs::schedule::schedule::Schedule::run";

#[cfg(windows)]
thread_local! {
    pub static TINY_GLADE: Lazy<HMODULE> = Lazy::new(|| unsafe {
        GetModuleHandleA(s!("tiny-glade.exe")).expect("could not get Tiny Glade module handle")
    });
}

extern "cdecl" fn run_schedule(schedule: *mut Schedule, world: *mut World) {
    let schedule = unsafe { &mut *schedule };
    let world = unsafe { &mut *world };

    SCHEDULE_RUN_DETOUR.read().as_ref().unwrap().call(schedule, world);

    let mut guard = TYPES.lock();
    let Some(types) = guard.as_mut() else {
        return;
    };

    for info in world.components().iter() {
        if let Some(type_id) = info.type_id() {
            types.insert(info.name(), type_id);
        }
    }
}

#[derive(Parser)]
pub struct Arguments {
    #[arg(default_value = "7")]
    pub delay: f32,
    #[arg(default_value = "dump.bin")]
    pub path: PathBuf,
}

#[ctor::ctor]
fn ctor() {
    let Ok(stream) = TcpStream::connect("127.0.0.1:8996") else {
        return;
    };

    tracing_subscriber::fmt().with_max_level(Level::DEBUG).with_writer(std::sync::Mutex::new(stream)).init();

    info!("Connected to injector process");

    thread::spawn(|| {
        if let Err(err) = fallible() {
            error!("Encountered error in exectution: {err}");
        }
    });
}

fn fallible() -> anyhow::Result<()> {
    let args = Arguments::parse_from(shell_words::split(&env::var("TINY_PINCH_ARGUMENTS")?)?);

    let glade_path: PathBuf = env::var("TINY_PINCH_GLADE_PATH")?.parse()?;
    let pdb_path = glade_path.parent().ok_or(anyhow!("Could not get parent directory of Tiny Glade"))?.join("tiny_glade.pdb");

    info!("Getting offset information");

    let mut offsets = Offsets::new();

    let pdb_file = BufReader::new(File::open(pdb_path)?);
    let mut pdb = PDB::open(pdb_file)?;

    let symbol_table = pdb.global_symbols()?;
    let address_map = pdb.address_map()?;

    let mut symbols = symbol_table.iter();

    while let Some(symbol) = symbols.next()? {
        if let Ok(SymbolData::Public(data)) = symbol.parse() {
            if !data.function {
                continue;
            }

            let mangled = &data.name.to_string();
            let name = demangle(mangled);

            let demangled_hashed = format!("{name:?}");
            let symbol = format!("{name:#?}");

            let Some(hash) = demangled_hashed.strip_prefix(&symbol) else {
                continue;
            };

            if hash.len() < 2 {
                continue;
            }

            let hash = &hash[2..];

            let offset = data.offset
                .to_rva(&address_map)
                .ok_or_else(|| anyhow!("Could not compute offset of: {demangled_hashed}"))?
                .0 as isize;

            offsets.add_offset(&symbol, hash, offset);

            debug!("Found offset of: {symbol} ({hash})");
        }
    }

    info!("Found {} offsets", offsets.len());

    let scheudle_run_offset = offsets.get_offset(SCHEDULE_RUN, None)
        .ok_or_else(|| anyhow!("Could not find offset for: {SCHEDULE_RUN}"))?;

    info!("Schedule Run Offset: {scheudle_run_offset}");

    TYPES.lock().replace(Types::new());

    unsafe {
        let function_ptr = TINY_GLADE.with(|tiny_glade| transmute(tiny_glade.0.byte_offset(scheudle_run_offset)));
        
        let detour = GenericDetour::<extern "cdecl" fn(*mut Schedule, *mut World)>::new(
            Function::from_ptr(function_ptr),
            run_schedule,
        )?;

        detour.enable()?;

        SCHEDULE_RUN_DETOUR.write().replace(detour);
    }
    
    let dump_file = BufWriter::new(File::create(args.path)?);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs_f32(args.delay));

        let Some(types) = TYPES.lock().take() else {
            error!("Could not get types after delay");
            exit(-1);
        };

        let dump = Dump::new(offsets, types);

        bincode::serialize_into(dump_file, &dump).expect("could not write dump");

        info!("Successfully dumped Tiny Glade!");
        info!("{} offsets {} type ids", dump.offsets().len(), dump.types().len());

        exit(0);
    });

    Ok(())
}
