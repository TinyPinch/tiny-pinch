pub mod ext;
pub mod map;
pub mod query;

use std::{env, mem::transmute, net::TcpStream};

pub use canopy_macros::*;
use bevy_app::App;
use lumberjack::Dump;
use once_cell::unsync::Lazy;
use retour::{Function, GenericDetour};
use thiserror::Error;
use tracing::{info, Level};
use windows::{core::s, Win32::{Foundation::HMODULE, System::LibraryLoader::GetModuleHandleA}};

pub mod prelude {
    pub use canopy_macros::*;
    pub use bevy_app;
    pub use bevy_ecs;
    pub use ctor;
    pub use lumberjack;
    pub use lazy_static;
    pub use retour;
    pub use clap;
    pub use tracing;
    pub use crate::{CanopyMod, CanopyError, ext::prelude::*};
}

lazy_static::lazy_static! {
    pub static ref DUMP: Dump = Dump::load_from(env::var("TINY_PINCH_DUMP_PATH").unwrap()).unwrap();
    pub static ref ARGUMENTS: Vec<String> = shell_words::split(&env::var("TINY_PINCH_ARGUMENTS").unwrap()).unwrap();
}

#[cfg(windows)]
thread_local! {
    pub static TINY_GLADE: Lazy<HMODULE> = Lazy::new(|| unsafe {
        GetModuleHandleA(s!("tiny-glade.exe")).expect("could not get Tiny Glade module handle")
    });
}

pub const BUILD_HOOK: &str = "<country_core::systems::main_camera::MainCameraPlugin as bevy_app::plugin::Plugin>::build";

#[derive(Debug, Error)]
pub enum CanopyError {
    #[error("type was not found in Tiny Glade")]
    TypeNotFound,
}

pub type Result<T> = std::result::Result<T, CanopyError>;

pub trait CanopyMod {
    type Arguments: clap::Parser;

    fn initialize(arguments: &Self::Arguments) -> Self;
    fn build(&self, arguments: &Self::Arguments, app: &mut App) -> crate::Result<()>;
}

pub fn initialize_logging() {
    let Ok(stream) = TcpStream::connect("127.0.0.1:8996") else {
        return;
    };

    tracing_subscriber::fmt().with_max_level(Level::DEBUG).with_writer(std::sync::Mutex::new(stream)).init();

    info!("Connected to injector process");
}

pub unsafe fn hook<F: Function>(symbol: &str, hash: Option<&str>, detour: F) -> GenericDetour<F> {
    let offset = DUMP.offsets().get_offset(symbol, hash).unwrap_or_else(|| panic!("Could not get offset for {symbol}"));

    let function_pointer = TINY_GLADE.with(|tiny_glade| transmute(tiny_glade.0.byte_offset(offset)));

    GenericDetour::new(
        F::from_ptr(function_pointer),
        detour,
    )
    .unwrap_or_else(|_| panic!("Could not hook {symbol}"))
}

pub unsafe fn hook_enable<F: Function>(symbol: &str, hash: Option<&str>, detour: F) -> GenericDetour<F> {
    let hooked = hook(symbol, hash, detour);

    hooked.enable().unwrap_or_else(|_| panic!("Could not enable hook {symbol}"));

    hooked
}
