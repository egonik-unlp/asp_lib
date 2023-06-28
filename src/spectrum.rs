use polars::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct Spectrum {
    pub filename: String,
    wavenumber_grid: Vec<f64>,
    transmittance_grid: Vec<f64>,
}

impl Spectrum {
    pub fn new(filename: String, wng: Vec<f64>, tng: Vec<f64>) -> Spectrum {
        Spectrum {
            filename: filename,
            wavenumber_grid: wng,
            transmittance_grid: tng,
        }
    }
    pub fn to_csv(&self, dest_folder: &str) -> Result<String, Box<dyn Error>> {
        let mut df: DataFrame = df!(
            "wavenumber" => &self.wavenumber_grid,
            "transmittance" => &self.transmittance_grid
        )?;
        let conv_filename = format!(
            "./{}/{}.csv",
	    dest_folder,
            &self.filename[..self.filename.len() - 4]
        );
        let mut file = std::fs::File::create(&conv_filename)?;
        CsvWriter::new(&mut file).finish(&mut df)?;
        Ok(String::from(&conv_filename))
    }
}
