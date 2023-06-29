use csv::Writer;
use std::{
    error::Error,
    fs::create_dir_all,
    path::Path,
};
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
        let orpath = Path::new(&self.filename);
        let folder = orpath.clone().parent().unwrap();
        let filename = orpath.clone().file_prefix().unwrap();
        let file = Path::new(&filename).with_extension("csv");
        let path = folder.join(dest_folder).join(file);

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
        Ok((&path).to_str().unwrap().to_owned())
    }
}
