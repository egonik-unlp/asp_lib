use crate::spectrum::Spectrum;
use itertools::Itertools;
use itertools_num::linspace;
use std::error::Error;
use std::fs;
use std::path::Path;
use walkdir::DirEntry;

pub fn extension_is_asp(filename: &String) -> bool {
    let path = Path::new(filename).extension();
    match path {
        Some(i) => i.to_str().unwrap_or("basura").to_lowercase().eq("asp"),
        None => false,
    }
}

pub fn handle_one_file(filename: &str) -> Result<Spectrum, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut contents = contents.lines();
    let (ln, hwn, lwn): (f64, f64, f64) = contents
        .next_chunk::<3>()
        .unwrap()
        .into_iter()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect_tuple()
        .ok_or("Archivo mal formateado")?;
    let contents = contents.into_iter().skip(3);

    let wng = linspace::<f64>(hwn, lwn, ln as usize).collect();

    let tnsg: Vec<f64> = contents
        .into_iter()
        .filter_map(|x| x.parse::<f64>().ok())
        .collect();
    let spec = Spectrum::new(filename.to_owned(), wng, tnsg);
    Ok(spec)
}

pub fn handle_folders(paths: Vec<DirEntry>, export_path: &str) {
    let basepath = Path::new(export_path);
    for foldpath in paths.into_iter() {
        let pth = basepath.join(foldpath.path());
        if pth.ne(basepath) {
            fs::create_dir_all(pth).unwrap();
        }
    }
}
