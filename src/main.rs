use clap::Parser;
use tiny_pinch::{launch, Arguments, GLADE_PATH};
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let args = Arguments::parse();

    info!("Tiny Glade path: {:?}", *GLADE_PATH);

    launch(args.mod_path, args.additional_arguments)
}
