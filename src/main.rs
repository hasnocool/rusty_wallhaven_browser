use eframe::egui;
use eframe::egui::{ColorImage, TextureHandle};
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::Class;
use crossbeam::thread;
use std::sync::mpsc;
use anyhow::{Result, Context};
use std::collections::HashMap;

#[derive(Default)]
struct AppState {
    wallpapers: Vec<(String, String)>, // (image_url, wallpaper_url)
    textures: Vec<Option<TextureHandle>>,
    image_size: f32, // Size of each wallpaper image
    columns: usize,   // Number of columns
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Wallhaven Browser",
        options,
        Box::new(|cc| {
            let mut app = AppState {
                image_size: 300.0,
                columns: 4,
                ..Default::default()
            };
            app.load_wallpapers("https://wallhaven.cc/", &cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

impl eframe::App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // ... (previous UI code remains the same)

            let available_width = ui.available_width();

            // Adjust image size based on available space
            let padding = 5.0; // Add some padding between images
            let image_width = (available_width - (self.columns as f32 + 1.0) * padding) / self.columns as f32;
            let image_height = self.image_size;
            let image_size = egui::vec2(image_width, image_height);

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("wallpapers_grid").spacing([padding, padding]).show(ui, |ui| {
                    for (i, texture_option) in self.textures.iter().enumerate() {
                        if i % self.columns == 0 && i != 0 {
                            ui.end_row();
                        }
                        if let Some(texture) = texture_option {
                            let image = egui::Image::new(texture);
                            if ui.add_sized(image_size, image).clicked() {
                                if let Some(wallpaper_url) = self.wallpapers.get(i).map(|(_, url)| url) {
                                    if let Err(e) = open::that(wallpaper_url) {
                                        eprintln!("Failed to open URL: {}", e);
                                    }
                                }
                            }
                        } else {
                            ui.add_sized(image_size, egui::Label::new("Loading..."));
                        }
                    }
                });
            });
        });
    }
}


impl AppState {
    fn load_wallpapers(&mut self, url: &str, ctx: &egui::Context) {
        let client = Client::new();
        if let Ok(response) = client.get(url).send() {
            if let Ok(document) = Document::from_read(response) {
                self.wallpapers.clear();
                self.textures.clear();

                let nodes: Vec<_> = document.find(Class("thumb"))
                    .filter_map(|node| {
                        let img_tag = node.find(Class("lazyload")).next()?;
                        let img_src = img_tag.attr("data-src")?;
                        let preview = node.find(Class("preview")).next()?;
                        let wallpaper_url = preview.attr("href")?;
                        Some((img_src.to_string(), wallpaper_url.to_string()))
                    })
                    .collect();

                let nodes_len = nodes.len(); // Take the length before moving nodes
                let (tx, rx) = mpsc::channel();

                thread::scope(|s| {
                    for (img_src, wallpaper_url) in nodes {
                        let tx = tx.clone();
                        let ctx = ctx.clone();
                        s.spawn(move |_| {
                            match load_image_from_url(&img_src, &ctx) {
                                Ok(texture) => tx.send((img_src, wallpaper_url, Some(texture))).unwrap(),
                                Err(_) => tx.send((img_src, wallpaper_url, None)).unwrap(),
                            }
                        });
                    }
                }).unwrap();

                for (img_src, wallpaper_url, texture) in rx.iter().take(nodes_len) {
                    self.wallpapers.push((img_src, wallpaper_url));
                    self.textures.push(texture);
                }
            }
        }
    }
}

fn load_image_from_url(url: &str, ctx: &egui::Context) -> Result<TextureHandle> {
    let response = reqwest::blocking::get(url)
        .with_context(|| format!("Failed to GET from '{}'", url))?;
    let bytes = response.bytes()
        .context("Failed to get bytes from response")?;
    let image = image::load_from_memory(&bytes)
        .context("Failed to load image from memory")?
        .to_rgba8();
    let size = [image.width() as _, image.height() as _];
    let color_image = ColorImage::from_rgba_unmultiplied(size, &image);
    Ok(ctx.load_texture(url, color_image, Default::default()))
}


