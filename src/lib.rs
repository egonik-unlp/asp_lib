#![feature(iter_next_chunk)]
#![feature(absolute_path)]
#![feature(path_file_prefix)]

pub mod helper_funcs;
mod spectra;
mod spectrum;
use helper_funcs::handle_one_file;

use spectra::Spectra;
use split_iter::Splittable;
use std::error::Error;
use std::path::{self, Path};

/// Parses a directory `path` (subdirectories included)., finds all .asp files contained
/// within, and generates the same folder structure with converted .csv files in `export_path`
/// # Examples
///
/// ```
/// use asp_lib::handle_many_spectra;
/// handle_many_spectra(".", "./exported files")
/// ```
pub fn handle_many_spectra(
    path: &str,
    export_path: &str,
    plot: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let spectra = Spectra::build_from_path(path, export_path)?;
    spectra.export_all(plot);
    Ok(String::from(path))
}

/// Converts a single .asp file into .csv. It also adds the corresponding wavenumber column
/// # Examples
///
/// ```
/// use asp_lib::handle_single_spectrum;
/// # to generate ./exported/file.csv
/// handle_single_spectrum("file.asp", "./exported")
/// ```
pub fn handle_single_spectrum(
    filepath: &str,
    savepath: &str,
    plot: bool,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut spectrum = handle_one_file(filepath)?;
    spectrum.to_csv(savepath)?;
    if plot {
        spectrum.plot()
    }
    Ok(String::from(filepath))
}
