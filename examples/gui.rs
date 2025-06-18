use std::path::Path;

use eframe::{egui, egui::Color32, egui::ColorImage};
use egui_extras::RetainedImage;
use goo::GooFile;

struct GooViewer {
    goo: Option<GooFile>,
    current_layer: usize,
    texture: Option<RetainedImage>,
}

impl Default for GooViewer {
    fn default() -> Self {
        Self {
            goo: None,
            current_layer: 0,
            texture: None,
        }
    }
}

impl GooViewer {
    fn load_file(&mut self, path: &Path) {
        match std::fs::read(path).and_then(|bytes| {
            GooFile::deserialize(&bytes)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        }) {
            Ok(goo) => {
                self.current_layer = 0;
                self.goo = Some(goo);
                self.update_texture();
            }
            Err(err) => {
                eprintln!("Failed to load {path:?}: {err}");
            }
        }
    }

    fn update_texture(&mut self) {
        let Some(goo) = &self.goo else { return };
        let Some(layer) = goo.layers.get(self.current_layer) else {
            return;
        };
        let img = layer.to_image(
            goo.header.x_resolution as u32,
            goo.header.y_resolution as u32,
        );
        let size = [img.width() as usize, img.height() as usize];
        let pixels = img
            .as_raw()
            .iter()
            .map(|&v| Color32::from_gray(v))
            .collect();
        let color = ColorImage { size, pixels };
        self.texture = Some(RetainedImage::from_color_image("layer", color));
    }
}

impl eframe::App for GooViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            if ui.button("Open .goo file...").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("Goo", &["goo"])
                    .pick_file()
                {
                    self.load_file(&path);
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(goo) = &self.goo {
                let mut changed = false;
                ui.horizontal(|ui| {
                    ui.label("Layer:");
                    let range = 0..=goo.layers.len() - 1;
                    changed |= ui
                        .add(egui::Slider::new(&mut self.current_layer, range))
                        .changed();
                    ui.label(format!("/ {}", goo.layers.len()));
                });
                if changed {
                    self.update_texture();
                }
                ui.separator();
                if let Some(tex) = &self.texture {
                    tex.show(ui);
                }
            } else {
                ui.label("Open a .goo file to begin.");
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Goo Viewer",
        options,
        Box::new(|_| Ok(Box::<GooViewer>::default())),
    )
    .unwrap();
}
