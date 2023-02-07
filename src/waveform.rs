use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;

pub struct Waveform {
    // Each point should be from 0 to 1
    series: Vec<f64>,
    source: String,
    x_units: String,
    y_units: String,

}

type CsvIterator = dyn Iterator<Item=Result<csv::StringRecord, csv::Error>>;

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
    pub fn read(&mut self, path: &PathBuf) -> Result<(), Box<dyn Error>> {
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
    fn load_series(&mut self, mut rdr: csv::Reader<BufReader<File>>) -> Result<(), Box<dyn Error>>{

        Ok(())
    }

    /// Load config given by the CSV
    fn load_settings(&mut self, rdr: &mut csv::StringRecordsIter<BufReader<File>>) {
        let r1 = rdr.nth(0).expect("Invalid CSV").unwrap();
        let l: usize = r1.get(1).expect("Invalid CSV").to_string()
            .parse::<usize>().expect("Invalid CSV");
        self.series = Vec::with_capacity(l); // preallocate the length of the series
        println!("Series length: {}", self.series.capacity());
    }
}