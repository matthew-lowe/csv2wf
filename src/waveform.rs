use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;
use colored::*;
use plotters::prelude::*;
use plotters::style::Color;
use crate::Cli;

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
        write!(f, "WaveformError: {}", self.details)
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


/// Waveform object, also stored the config whether you like it or not
pub struct Waveform {
    // User Config
    args: Rc<Cli>,
    // CSV Config
    // Each point should be from 0 to 1
    pub series_x: Vec<f64>,
    pub series_y: Vec<f64>,
    max_x: f64,
    min_x: f64,
    max_y: f64,
    min_y: f64,
    pub source: String,
    pub x_units: String,
    pub y_units: String,
    pub length: i32,
    pub caption: String,
    pub scale_x: f64,
    pub scale_y: f64,
}

type CsvRdr<'a> = csv::Reader<BufReader<File>>;

/// Create a WaveformError, Box it and wrap in an Err
macro_rules! wfm_err {
    ($details:expr) => {
        Err(Box::new(WaveformError::new($details)))
    }
}

impl Waveform {
    pub fn new(args: Rc<Cli>) -> Self {
        Waveform {
            // User Config
            args,
            // CSV Config
            series_x: vec![],
            series_y: vec![],
            max_x: f64::MIN,
            min_x: f64::MAX,
            max_y: f64::MIN,
            min_y: f64::MAX,
            source: String::new(),
            x_units: String::new(),
            y_units: String::new(),
            length: 0,
            caption: String::new(),
            scale_x: 1f64,
            scale_y: 1f64,
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
        self.load_series(&mut rdr)?;

        Ok(())
    }

    /// Load the waveform data (assumes uniform sample distribution)
    fn load_series(&mut self, rdr: &mut CsvRdr) -> UnitResult {
        let mut iter = rdr.records();
        iter.reader_mut().seek(csv::Position::new())?;

        let mut i = 0;
        for result in iter {
            let record = result?;
            let cell_x = record.get(3).ok_or(WaveformError::new(errors::INVALID_CSV))?;
            let cell_y = record.get(4).ok_or(WaveformError::new(errors::INVALID_CSV))?;
            if cell_x != "" && cell_y != "" {
                let value_x: f64 = cell_x.parse()?;
                let value_y: f64 = cell_y.parse()?;
                self.series_x.insert(i, value_x);
                self.series_y.insert(i, value_y);

                if value_x > self.max_x {
                    self.max_x = value_x;
                }

                if value_x < self.min_x {
                    self.min_x = value_x;
                }

                if value_y > self.max_y {
                    self.max_y = value_y;
                }

                if value_y < self.min_y {
                    self.min_y = value_y;
                }
            }
            i += 1;
        }

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
        self.length = l.parse()?;
        self.caption = load_setting!("Note")?;
        let s_x: String = load_setting!("Horizontal Scale")?;
        self.scale_x = s_x.parse()?;
        let s_y: String = load_setting!("Vertical Scale")?;
        self.scale_y = s_y.parse()?;

        self.series_x = Vec::with_capacity(self.length as usize);
        self.series_y = Vec::with_capacity(self.length as usize);

        Ok(())
    }

    /// Find a setting by name and do the type conversion, returns WaveformError if either of
    /// these fail.
    pub fn find_setting(&mut self, rdr: &mut CsvRdr, name: &str) -> Result<String, Box<dyn Error>> {
        // TODO: fix this mess :(
        let mut iter = rdr.records();
        // god gave us no choice
        iter.reader_mut().seek(csv::Position::new())?;

        for result in iter {
            if let Ok(record) = result {
                for i in 0..record.len() {
                    if let Some(cell) = record.get(i as usize) {
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

    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let root = BitMapBackend::new("out.png", (600, 400)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(self.caption.as_str(), ("sans-serif", 45).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(self.min_x/self.scale_x..self.max_x/self.scale_x,
                                self.min_y/self.scale_y..self.max_y/self.scale_y)?;

        chart.configure_mesh()
            .x_desc(format!("{} ({})", self.args.x_label, self.x_units))
            .y_desc(format!("{} ({})", self.args.y_label, self.y_units))
            .draw()?;

        chart
            .draw_series(LineSeries::new(
                (0..self.length).map(|i|
                    (self.series_x[i as usize]/self.scale_x, self.series_y[i as usize]/self.scale_y)),
                &RED,
            ))?
            .label(self.source.as_str())
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.8))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        Ok(())
    }
}