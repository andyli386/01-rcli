use clap::Parser;
use rcli::{
    opts::{Opts, SubCommand},
    process::process_csv,
};

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
