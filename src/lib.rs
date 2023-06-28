#![feature(iter_next_chunk)]
mod helper_funcs;
mod spectra;
mod spectrum;
use spectra::Spectra;
use helper_funcs::handle_one_file;
use split_iter::Splittable;
use std::error::Error;
use std::path::Path;


/// Parses a directory `path` (subdirectories included)., finds all .asp files contained
/// within, and generates the same folder structure with converted .csv files in `export_path`
/// # Examples
///
/// ```
/// use asp_lib::handle_many_spectra;
/// handle_many_spectra(".", "./exported files")
/// ```
pub fn handle_many_spectra(path : &str , export_path : &str) -> () {
    let spectra = Spectra::build_from_path(path, export_path).unwrap();
    spectra.export_all();
}

/// Converts a single .asp file into .csv. It also adds the corresponding wavenumber column
/// # Examples
///
/// ```
/// use asp_lib::handle_single_spectrum;
/// # to generate ./exported/file.csv
/// handle_single_spectrum("file.asp", "./exported")
/// ```
pub fn handle_single_spectrum(filepath : &str, savepath: &str) -> () {
    let spectrum = handle_one_file(filepath).expect("Problema leyendo el archivo");
    spectrum.to_csv(savepath).expect("Error guardando el archivo");
}