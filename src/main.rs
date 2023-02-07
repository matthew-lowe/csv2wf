use clap::Parser;
use colored::*;

mod waveform;

#[derive(Parser)]
struct Cli {
    /// Path of the .csv file to read
    path: std::path::PathBuf,
    /// The channel of the waveform to display
    #[arg(short, long, default_value_t = String::from("CH1"))]
    channel: String,
}

fn main() {
    let args: Cli = Cli::parse();
    let mut wf = waveform::Waveform::new();
    wf.read(&args.path);
    println!("{:?}", wf.series);

}
