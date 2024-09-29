pub mod glade_path;

use std::{env, path::PathBuf};

use clap::Parser;
use glade_path::get_glade_dir;
use lazy_static::lazy_static;

#[cfg(windows)]
const EXECUTABLE_NAME: &str = "tiny-glade.exe";
#[cfg(unix)]
const EXECUTABLE_NAME: &str = "tiny-glade";

lazy_static! {
    pub static ref GLADE_DIR: PathBuf = get_glade_dir();
    pub static ref GLADE_PATH: PathBuf = GLADE_DIR.join(env::var("TINY_PINCH_GLADE_EXE").unwrap_or_else(|_| String::from(EXECUTABLE_NAME)));
}

#[derive(Parser)]
/// Tiny Glade mod loader
pub struct Arguments {
    /// Path to the mod
    pub mod_path: PathBuf,
    /// Additional arguments passed to the mod
    #[arg(last(true))]
    pub additional_arguments: Vec<String>,
}

#[cfg(windows)]
pub use windows::launch;
#[cfg(unix)]
pub use unix::launch;

#[cfg(windows)]
mod windows {
    use std::{iter::once, net::TcpListener, path::Path, process::Command};

    use dll_syringe::{process::{OwnedProcess, Process}, Syringe};
    use tracing::info;

    use crate::{GLADE_DIR, GLADE_PATH};

    pub fn launch(mod_path: impl AsRef<Path>, additional_arguments: Vec<String>) -> anyhow::Result<()> {
        let mod_path = mod_path.as_ref();
    
        let mut tiny_glade_command = Command::new(&*GLADE_PATH);
    
        tiny_glade_command.current_dir(&*GLADE_DIR);
    
        let arguments = once(mod_path.to_string_lossy().to_string()).chain(additional_arguments.into_iter());
    
        tiny_glade_command.env("TINY_PINCH_ARGUMENTS", shell_words::join(arguments));
        tiny_glade_command.env("TINY_PINCH_GLADE_PATH", GLADE_PATH.as_os_str());
    
        let tiny_glade_process = tiny_glade_command.spawn()?;
    
        info!("Launched Tiny Glade process: {}", tiny_glade_process.id());
    
        let target_process = OwnedProcess::from_child(tiny_glade_process);
        let syringe = Syringe::for_process(target_process);

        info!("You're going to feel a tiny pinch.");
    
        let result = inject(&syringe, mod_path);

        syringe.process().kill()?;

        result
    }
    
    pub fn inject(syringe: &Syringe, mod_path: &Path) -> anyhow::Result<()> {
        let listener = TcpListener::bind("127.0.0.1:8996")?;

        let _injected_payload = syringe.inject(&mod_path)?;
        info!("Injected successfully");

        let (mut stream, address) = listener.accept()?;
        info!("Connected to process at: {address}");

        let mut stdout = std::io::stdout();
        std::io::copy(&mut stream, &mut stdout)?;
        
        Ok(())
    }    
}

#[cfg(unix)]
mod unix {
    use std::path::Path;
    
    pub fn launch(mod_path: impl AsRef<Path>, additional_arguments: Vec<String>) -> anyhow::Result<()> {
        unimplemented!("Tiny Pinch currently only supports Windows.");
    }
}
