use eframe::egui;
use log::error;

struct ImFlock {
}

impl ImFlock {
    fn new() -> Self {
        Self{
        }
    }
}

impl eframe::App for ImFlock {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Save graph").clicked() {
            }
        });

        ctx.request_repaint();
    }
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
