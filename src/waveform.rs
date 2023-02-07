use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;
use colored::*;

type UnitResult = Result<(), Box<dyn Error>>;


enum MetricScale {
    G = 9,
    M = 6,
    K = 3,
    m = -3,
    u = -6,
    n = -9,
    p = -12,
}


#[derive(Debug)]
pub struct WaveformError {
    details: String,
}

impl WaveformError {
    fn new(msg: &str) -> Self {
        WaveformError {
            details: msg.to_string()
        }
    }
}

impl Display for WaveformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "CsvReadError: {}", self.details)
    }
}

impl Error for WaveformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        self.details.as_str()
    }
}

mod errors {
    pub static INVALID_CSV: &'static str = "Invalid CSV Input";
    pub static NOT_FOUND: &'static str = "Record not found";
}


#[derive(Debug)]
pub struct Waveform {
    // Each point should be from 0 to 1
    series: Vec<f64>,
    pub source: String,
    pub x_units: String,
    pub y_units: String,
    pub length: i32,
}

type CsvRdr<'a> = csv::Reader<BufReader<File>>;

/// Create a WaveformError, Box it and wrap in an Err
macro_rules! wfm_err {
    ($details:expr) => {
        Err(Box::new(WaveformError::new($details)))
    }
}

impl Waveform {
    pub fn new() -> Self {
        Waveform {
            series: vec![],
            source: String::new(),
            x_units: String::new(),
            y_units: String::new(),
            length: 0,
        }
    }

    /// Load the CSV and convert into a waveform
    pub fn read(&mut self, path: &PathBuf) -> UnitResult {
        // Since this program reads a file stated by the user before execution, an incorrect file is
        // unrecoverable so the program panics. There will always be a path given if the program
        // gets here
        let file = OpenOptions::new().read(true).open(path)
            .expect(format!("Unable to open file {}", path.to_str().unwrap()).as_str());


        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(b',')
            .has_headers(false)
            .flexible(true)
            .from_reader(BufReader::new(file));


        self.load_settings(&mut rdr)?;

        Ok(())
    }

    /// Load the waveform data
    fn load_series(&mut self, rdr: &mut CsvRdr) -> UnitResult {

        Ok(())
    }

    /// Load config given by the CSV
    fn load_settings(&mut self, rdr: &mut CsvRdr) -> UnitResult {
        macro_rules! load_setting {
            ($n:expr) => {
                match self.find_setting(rdr, $n) {
                    Ok(v) => Ok(v),
                    _ => wfm_err!(errors::NOT_FOUND),
                }
            }
        }

        self.source = load_setting!("Source")?;
        self.x_units = load_setting!("Horizontal Units")?;
        self.y_units = load_setting!("Vertical Units")?;
        let l: String = load_setting!("Record Length")?;
        println!("LENGTH: {}", l);

        Ok(())
    }

    /// Find a setting by name and do the type conversion, returns WaveformError if either of
    /// these fail. The name must match exact, case sensitive (depth default 13 across, 17 down)
    pub fn find_setting(&mut self, rdr: &mut CsvRdr, name: &str) -> Result<String, Box<dyn Error>> {
        // TODO: fix this mess :(
        let mut iter = rdr.records();
        // god gave us no choice
        iter.reader_mut().seek(csv::Position::new())?;

        let mut j = 0;
        for result in iter {
            if let Ok(record) = result {
                for i in 0..record.len() {
                    j += 1;
                    if let Some(cell) = record.get(i as usize) {
                        if j < 10 {
                            println!("Searching: {}", cell.red());
                            println!("Name = {}", name.blue());
                        }
                        if cell == name {
                            let data = record.get(i + 1 as usize).unwrap().clone();
                            return Ok(String::from(data));
                        }
                    }
                }
            }
        }

        wfm_err!(errors::INVALID_CSV)
    }

}