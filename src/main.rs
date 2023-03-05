use anyhow::Result;
use job_convert::convert_file;

use clap::Parser;

#[derive(Parser)]
#[command(author, about = "Convert from VBR Get to Post", long_about = None)]
struct Cli {
    #[arg(short, long, required = true)]
    read_path: String,

    #[arg(short, long, required = true)]
    write_path: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    convert_file(cli.read_path, cli.write_path)?;

    Ok(())
}
