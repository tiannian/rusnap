use clap::Parser;

mod build;
mod command;
mod new;
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
    let CargoCli::Rusnap(args) = CargoCli::parse();

    args.rusnap.execute().unwrap();
}
