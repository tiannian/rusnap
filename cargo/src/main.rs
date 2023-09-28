use clap::{Args, Parser, Subcommand};

mod status;

#[derive(Parser)] // requires `derive` feature
enum CargoCli {
    Rusnap(DeriveArgs),
}

#[derive(Debug, Subcommand)]
enum Rusnap {
    Status,
    New,
    Build,
    Start,
    Publish,
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct DeriveArgs {
    #[command(subcommand)]
    rusnap: Rusnap,
}

fn main() {
    let CargoCli::Rusnap(args) = CargoCli::parse();
}
