use std::{env, path::PathBuf};

use steamlocate::SteamDir;
use tracing::info;

pub fn get_glade_dir() -> PathBuf {
    match env::var("TINY_PINCH_GLADE_PATH") {
        Ok(path) => PathBuf::from(path),
        Err(_) => detect_glade_dir(),
    }
}

pub fn detect_glade_dir() -> PathBuf {
    info!("Detecting Tiny Glade directory");

    let mut steamdir = SteamDir::locate().expect("could not locate steam directory");

    let app = steamdir
        .app(&2198150)
        .expect("could not locate Tiny Glade directory");

    app.path.clone()
}
