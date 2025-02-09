use eframe::egui;
use log::error;
use std::path::PathBuf;

struct DirectoryItem {
}

struct ImFlock {
    base_dir: PathBuf,
    directory_items: Vec<DirectoryItem>
}

impl ImFlock {
    fn new() -> Self {
        Self{
            base_dir: "".into(),
            directory_items: vec![]
        }
    }
}

impl eframe::App for ImFlock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            display_img(ctx, ui);
        });

        ctx.request_repaint();
    }
}

fn display_img(ctx: &egui::Context, ui: &mut egui::Ui) {
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

