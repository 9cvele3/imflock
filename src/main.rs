use eframe::egui;
use log::error;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::PathBuf;

struct ImFlock {
    base_dir: PathBuf,
    images: Vec<PathBuf>,
    current_img_ind: u32,
}

impl ImFlock {
    fn new() -> Self {
        Self {
            base_dir: "".into(),
            images: vec![],
            current_img_ind: 0,
        }
    }

    fn display_img(&self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let valid_ind =
            self.current_img_ind >= 0 && self.current_img_ind < self.images.len() as u32;

        if valid_ind {}
    }
}

impl eframe::App for ImFlock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.display_img(ctx, ui);
        });

        ctx.request_repaint();
    }
}

fn load_image_from_path(path: &std::path::Path) -> Result<egui::ColorImage, image::ImageError> {
    let mut reader = image::io::Reader::open(path)?;
    load_egui_image_from_image_reader(reader)
}

fn load_image_from_bytes(data: &[u8], name: &str) -> Result<egui::ColorImage, image::ImageError> {
    println!("loading from {} bytes", data.len());
    let format = image::ImageFormat::from_path(name)?;
    let mut reader = image::io::Reader::with_format(Cursor::new(data), format);
    load_egui_image_from_image_reader(reader)
}

fn load_egui_image_from_image_reader<R: Read + BufRead + Seek>(
    reader: image::io::Reader<R>,
) -> Result<egui::ColorImage, image::ImageError> {
    let image = reader.decode()?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let px = image_buffer.as_flat_samples();
    let pixels = px
        .as_slice()
        .chunks_exact(4)
        .map(|p| {
            let lr = p[0];
            let lg = p[1];
            let lb = p[2];
            egui::Color32::from_rgb(lr, lg, lb)
        })
        .collect();
    let image = egui::ColorImage { size, pixels };
    Ok(image)
}

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 400.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ImFlock",
        native_options,
        Box::new(move |_cc| Box::new(ImFlock::new())),
    )
    .unwrap_or_else(|e| error!("An error occured {}", e));
}
