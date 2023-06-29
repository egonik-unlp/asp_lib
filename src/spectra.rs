use crate::spectrum::Spectrum;
use crate::*;
use helper_funcs::*;
use walkdir::WalkDir;
use std::io:: ErrorKind;
#[derive(Debug)]
pub struct Spectra {
    pub data: Vec<Spectrum>,
    pub export_path: String,
}

impl Spectra {
    pub fn build_from_path(path: &str, export_path: &str) -> Result<Spectra, Box<dyn Error>> {
        match Path::new(&path).exists() {
            true => Ok(()),
            false => Err(std::io::Error::new(ErrorKind::NotFound,  format!("No se hallo el directorio {}", path) ))
        }? ;
        
        let walker = WalkDir::new(path);
        let (files, dirs) = walker
            .into_iter()
            .split(|path| path.as_ref().unwrap().path().is_dir());

        let spectral_files = files
            .into_iter()
            .map(|x| x.unwrap().path().display().to_string())
            .filter(|x| extension_is_asp(x))
            .collect::<Vec<_>>();

        let newly_created_folders = dirs
            .into_iter()
            .filter(|dir| {
                dir.as_ref()
                    .unwrap()
                    .path()
                    .into_iter()
                    .any(|path_name| !path_name.eq(export_path))
            })
            .filter(|entry| {
                spectral_files
                    .clone()
                    .iter()
                    .map(|st| Path::new(st).parent().unwrap())
                    .any(|fp| fp.eq(entry.as_ref().unwrap().path()))
            })
            .map(|node| node.unwrap())
            .collect::<Vec<_>>();
        handle_folders(newly_created_folders, export_path);

        let spectrum_vector = spectral_files
            .into_iter()
            .map(|x| handle_one_file(&x).unwrap())
            .collect::<Vec<_>>();
        Ok(Spectra {
            data: spectrum_vector,
            export_path: export_path.to_owned(),
        })
    }
    pub fn export_all(self) -> () {
        for file in self.data.into_iter() {
            println!("leyendo archivo {}", file.filename);
            let filename = file.to_csv(&self.export_path);
            match filename {
                Ok(dato) => println!("Exportado como {}", dato),
                Err(e) => println!("error => {:?}", e),
            }
        }
    }
}
