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

    // Validate arguments
    if !args.model_path.is_dir() {
        anyhow::bail!("Model path should be a valid directory")
    }

    println!("Importing from path: {}", args.model_path.display());
    import(&args.model_path)?;

    Ok(())
}
