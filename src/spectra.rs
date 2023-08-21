use crate::spectrum::Spectrum;
use crate::*;
use helper_funcs::*;
use itertools::Itertools;
use split_iter::Splittable;
use std::io::Error as IOError;
use std::io::ErrorKind;
use walkdir::WalkDir;
#[derive(Debug)]
pub struct Spectra {
    pub data: Vec<Spectrum>,
    pub export_path: String,
}

impl Spectra {
    pub fn build_from_path(path: &str, export_path: &str) -> Result<Spectra, Box<dyn Error>> {
        let path = Path::new(&path);
        match path.exists() {
            true => Ok(()),
            false if path.metadata().unwrap().permissions().readonly() => Err(IOError::new(
                ErrorKind::PermissionDenied,
                format!(
                    "No se puede acceder al directorio {} porque no tiene permiso de escritura",
                    path.to_str().unwrap()
                ),
            )),
            false => Err(std::io::Error::new(
                ErrorKind::NotFound,
                format!("No se hallo el directorio {}", path.to_str().unwrap()),
            )),
        }?;
        let walker = WalkDir::new(path);
        let (files, dirs) = walker.into_iter().split(|path| {
            println!(" split iterator {:?}", path.as_ref().unwrap());
            let splitter = path.as_ref().unwrap().path().is_dir();
            println!("{splitter}");
            splitter
        });

        let spectral_files = files
            .into_iter()
            .map(|x| {
                x.unwrap()
                    .path()
                    .strip_prefix(path)
                    .map(|x| {
                        println!("post strip prefix{:?}", x);
                        x
                    })
                    .unwrap()
                    .display()
                    .to_string()
            })
            .filter(|x| extension_is_asp(x))
            .collect::<Vec<_>>();
        println!("SPF => {spectral_files:?}");
        let newly_created_folders = dirs
            .into_iter()
            .map(|folder_path| {
                let aspath = folder_path.unwrap();
                let individual_path = aspath.path().strip_prefix(path).unwrap();
                path.join(export_path).join(individual_path)
            })
            .collect_vec();
        handle_folders(newly_created_folders, export_path);

        let spectrum_vector = spectral_files
            .into_iter()
            .map(|x| {
                let full_pathname = format!("{}/{}", path.display().to_string(), x);
                handle_one_file(&full_pathname, Some(path.display().to_string())).unwrap()
            })
            .collect::<Vec<_>>();
        Ok(Spectra {
            data: spectrum_vector,
            export_path: export_path.to_owned(),
        })
    }

    pub fn export_all(self) -> () {
        println!("ES NUEVO");
        for mut file in self.data.into_iter() {
            println!("leyendo archivo {}", file.filename);
            let filename = file.to_csv(&self.export_path);
            match filename {
                Ok(dato) => println!("Exportado como {}", dato),
                Err(e) => println!("error => {:?}", e),
            }
        }
    }
}
