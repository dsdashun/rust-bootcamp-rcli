// rcli csv -i input.csv -o output.json --header -d ','
use clap::Parser;
use rcli::{CmdExecutor, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
    opts.cmd.execute().await?;
    Ok(())
}
