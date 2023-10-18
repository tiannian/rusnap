use clap::Parser;
use log::LevelFilter;

mod build;
mod command;
mod new;
mod serve;
mod start;
mod status;
mod utils;

#[derive(Parser)] // requires `derive` feature
enum CargoCli {
    Rusnap(DeriveArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct DeriveArgs {
    #[command(subcommand)]
    rusnap: command::Rusnap,
}

fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();

    let CargoCli::Rusnap(args) = CargoCli::parse();

    args.rusnap.execute().unwrap();
}
