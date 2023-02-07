use std::error::Error;
use std::fmt::{Display, Formatter, write};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;
use crate::waveform::errors::INVALID_CSV;


type UnitResult = Result<(), Box<dyn Error>>;


enum UnitScale {
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
}


#[derive(Debug)]
pub struct Waveform {
    // Each point should be from 0 to 1
    series: Vec<f64>,
    source: String,
    x_units: String,
    y_units: String,

}

type CsvRdr<'a> = csv::StringRecordsIter<'a, BufReader<File>>;

impl Waveform {
    pub fn new() -> Self {
        Waveform {
            series: vec![],
            source: String::new(),
            x_units: String::new(),
            y_units: String::new(),
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


        self.load_settings(&mut rdr.records());

        Ok(())
    }

    /// Load the waveform data
    fn load_series(&mut self, mut rdr: csv::Reader<BufReader<File>>) -> UnitResult {

        Ok(())
    }

    /// Load config given by the CSV
    fn load_settings(&mut self, rdr: &mut CsvRdr) -> UnitResult {
        let r1 = rdr.nth(0).ok_or(WaveformError::new(errors::INVALID_CSV))??;
        let l: usize = r1.get(1).expect("Invalid CSV").to_string()
            .parse::<usize>().expect("Invalid CSV");
        self.series = Vec::with_capacity(l); // preallocate the length of the series
        println!("Series length: {}", self.series.capacity());


        Ok(())
    }

    /// Find a setting by name and do the type conversion, returns WaveformError if either of
    /// these fail. The name must match exact, case sensitive
    fn find_setting<T>(&mut self, rdr: &mut CsvRdr, name: &str) -> Result<T, Box<dyn Error>> {


        Err(Box::new(WaveformError::new(errors::INVALID_CSV)))
    }
}