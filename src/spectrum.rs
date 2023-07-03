use csv::Writer;

use std::path::PathBuf;
use std::{error::Error, fs::create_dir_all, path::Path};

use plotly::common::{Mode, Title};
use plotly::layout::{Axis, Layout};
use plotly::{ImageFormat, Plot, Scatter};

#[derive(Debug)]
pub struct Spectrum {
    pub filename: String,
    pub wavenumber_grid: Vec<f64>,
    pub transmittance_grid: Vec<f64>,
    pub filepath: Option<String>,
}

impl Spectrum {
    pub fn new(filename: String, wng: Vec<f64>, tng: Vec<f64>) -> Spectrum {
        Spectrum {
            filename: filename,
            wavenumber_grid: wng,
            transmittance_grid: tng,
            filepath: None,
        }
    }
    #[cfg(feature = "plotting")]
    pub fn plot(&self) -> () {
        let name_as_path = Path::new(&self.filename)
            .file_prefix()
            .unwrap()
            .to_str()
            .unwrap();
        let mut plot = Plot::new();
        let trace = Scatter::new(
            self.wavenumber_grid.clone(),
            self.transmittance_grid.clone(),
        )
        .mode(Mode::Lines);
        plot.add_trace(trace);
        let lyt = Layout::new()
            .title(Title::new(&name_as_path))
            .x_axis(Axis::new().title(Title::new("wavenumber")))
            .y_axis(Axis::new().title(Title::new("Transmittance (U.A)")));
        plot.set_layout(lyt);
        let mut filename = PathBuf::from(&self.filepath.as_ref().unwrap());
        filename.set_extension("png"); // let filename = format!("{}.png", &name_as_path);
        println!("para imagen {:?}", filename);
        plot.write_image(filename, ImageFormat::PNG, 800, 600, 1.0);
    }

    pub fn to_csv(&mut self, dest_folder: &str) -> Result<String, Box<dyn Error>> {
        let orpath = Path::new(&self.filename);
        let folder = orpath.clone().parent().unwrap();
        let filename = orpath.clone().file_prefix().unwrap();
        let file = Path::new(&filename).with_extension("csv");
        let path = folder.join(dest_folder).join(file);
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
        Ok((&path).to_str().unwrap().to_owned())
    }
}
