use csv::Writer;
use std::fs::OpenOptions;
use std::io::Write;
use std::{error::Error, fs::create_dir_all, path::Path};
#[derive(Debug)]
pub struct Spectrum {
    pub filename: String,
    pub wavenumber_grid: Vec<f64>,
    pub transmittance_grid: Vec<f64>,
    pub filepath: Option<String>,
    pub basepath: Option<String>,
}

impl Spectrum {
    pub fn new(filename: String, wng: Vec<f64>, tng: Vec<f64>, bp: Option<String>) -> Spectrum {
        Spectrum {
            filename: filename,
            wavenumber_grid: wng,
            transmittance_grid: tng,
            filepath: None,
            basepath: bp,
        }
    }
    pub fn to_csv(&mut self, dest_folder: &str) -> Result<String, Box<dyn Error>> {
        let orpath = Path::new(&self.filename);
        let folder = orpath.clone().parent().unwrap();
        let filename = orpath.clone().file_prefix().unwrap();
        let file = Path::new(&filename).with_extension("csv");
        let path = match &self.basepath {
            None => folder.join(dest_folder).join(file),
            Some(basepath) => {
                let path_distinto = folder.strip_prefix(&basepath).unwrap();
                let df_as_path = Path::new(dest_folder);
                let bp_as_path = Path::new(&basepath);
                bp_as_path.join(df_as_path).join(path_distinto).join(file)
            }
            
        };
        let mut logfile = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("{}/log_files_generados.txt", folder.display().to_string()))
            .expect("no pude generar/abrir logfile");

        self.filepath = Some(
            path.to_str()
                .ok_or(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "se rompio el file",
                ))?
                .to_owned(),
        );
        if let Some(prnt) = path.parent() {
            if !prnt.is_dir() {
                create_dir_all(prnt).expect(&format!(
                    "Se necesito crear el directorio raiz {} pero esto no fue posible",
                    prnt.display()
                ));
            }
        }
        let mut wtr = Writer::from_path(&path)?;
        wtr.write_record(&["wavenumber", "transmittance"])?;
        self.wavenumber_grid
            .clone()
            .into_iter()
            .zip(self.transmittance_grid.clone())
            .for_each(|(wn, tr)| wtr.write_record(&[wn.to_string(), tr.to_string()]).unwrap());
        wtr.flush()?;
        println!(
            "se logro exportar exitosamente el archivo {}",
            &path.to_str().unwrap()
        );
        logfile
            .write(format!("{}\n", &path.to_str().unwrap()).as_bytes())
            .expect("No pude escribir logs");
        Ok((&path).to_str().unwrap().to_owned())
    }
}
