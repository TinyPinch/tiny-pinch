use clap::Parser;
use tiny_pinch::{launch, Arguments, GLADE_PATH};
use tracing::info;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Arguments::parse();

    info!("Tiny Glade path: {:?}", *GLADE_PATH);

    launch(args.mod_path, args.additional_arguments)
}
