use ::egui::Color32;
use eframe::egui;
use log::error;
use std::collections::HashSet;
use std::fs;
use std::io::BufRead;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::PathBuf;

struct ImFlock {
    base_dir: PathBuf,
    images: Vec<PathBuf>,
    directories: HashSet<String>,
    current_img_ind: u32,
    target_dir: String,
}

impl ImFlock {
    fn new() -> Self {
        //let base_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("dataset");
        let base_dir = PathBuf::from(std::env::current_dir().unwrap());

        let images = fs::read_dir(&base_dir)
            .unwrap()
            .filter_map(|item| {
                if item.is_ok() {
                    Some(item.unwrap().path())
                } else {
                    None
                }
            })
            .collect();

        let directories = HashSet::new();

        Self {
            base_dir,
            images,
            directories,
            current_img_ind: 0,
            target_dir: "".to_owned(),
        }
    }

    fn display_img(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let valid_ind =
            self.current_img_ind >= 0 && self.current_img_ind < self.images.len() as u32;

        if valid_ind {
            let img_path = &self.images[self.current_img_ind as usize];

            if let Ok(img) = load_image_from_path(img_path) {
                let texture = ctx.load_texture(format!("thumb"), img, egui::TextureOptions::LINEAR);
                let sized_texture = egui::load::SizedTexture::from_handle(&texture);

                ui.add(egui::Label::new(img_path.as_os_str().to_str().unwrap()));

                ui.horizontal(|ui| {
                    ui.label("Move to directory: ");
                    let singleline = egui::TextEdit::singleline(&mut self.target_dir);
                    let response = ui.add(singleline);

                    let enter_pressed = response.ctx.input(|i| i.key_down(egui::Key::Enter));

                    let popup_id = ui.make_persistent_id("my_unique_id");

                    if response.clicked() {
                        ui.memory_mut(|mem| mem.toggle_popup(popup_id));
                    }

                    let move_img_to_folder = |img_filename, dir| {};

                    if enter_pressed {
                        self.directories.insert(self.target_dir.clone());
                        move_img_to_folder(img_path, self.target_dir.clone());
                    }

                    egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
                        ui.set_min_width(200.0); // if you want to control the size
                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill =
                            eframe::egui::Color32::GRAY;

                        for dir in self.directories.iter() {
                            if true || dir.starts_with(&self.target_dir) {
                                if ui.button(dir).clicked() {
                                    self.target_dir = dir.to_string();
                                    move_img_to_folder(img_path, dir.clone());
                                }
                            }
                        }
                    });
                });

                let img_widget = egui::Image::new(sized_texture)
                                            .maintain_aspect_ratio(true)
                                            .max_size(egui::Vec2{x: 300.0, y: 300.0});
                ui.add(img_widget);
            }

            if ctx.input(|input_state| input_state.key_pressed(egui::Key::ArrowLeft)) {
                if self.current_img_ind > 0 {
                    self.target_dir = "".to_owned();
                    self.current_img_ind -= 1;
                }
            }

            if ctx.input(|input_state| input_state.key_pressed(egui::Key::ArrowRight)) {
                if self.current_img_ind + 1 < self.images.len() as u32 {
                    self.target_dir = "".to_owned();
                    self.current_img_ind += 1;
                }
            }
        }
    }
}

impl eframe::App for ImFlock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new(format!("Directory: {}", self.base_dir.display())));
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
