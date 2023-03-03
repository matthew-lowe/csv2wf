use std::rc::Rc;
use clap::Parser;
use colored::*;

mod waveform;

#[derive(Parser)]
pub struct Cli {
    /// Path of the .csv file to read
    path: std::path::PathBuf,
    /// The channel of the waveform to display
    #[arg(short, long, default_value_t = String::from("CH1"))]
    channel: String,
    /// The y-axis label
    #[arg(short, long, default_value_t = String::from("Y"))]
    y_label: String,
    /// The x-axis label
    #[arg(short, long, default_value_t = String::from("X"))]
    x_label: String<>
}

fn main() {
    let args: Rc<Cli> = Rc::new(Cli::parse());
    let mut wf = waveform::Waveform::new(args.clone());
    wf.read(&args.path);

    println!("{:?}", wf.render());

}
