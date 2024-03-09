use anyhow::Result;
use clap::Parser;
use rfastspeech_import::import;

/// Tool for converting a model to RFastSpeech format
#[derive(Parser, Debug)]
#[command(version, about, long_about = None, name = "RFastSpeech model converter")]
struct Args {
    /// Input model path
    #[arg(short, long)]
    model_path: std::path::PathBuf,

    /// Output path
    #[arg(short, long)]
    output_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    println!("Importing from path: {}", args.model_path.display());
    let _model = import(&args.model_path)?;

    Ok(())
}
