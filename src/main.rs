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
    search_query: String,
    wallpapers: Vec<(String, String)>, // (image_url, wallpaper_url)
    textures: HashMap<String, Option<TextureHandle>>, // Map image_url to TextureHandle
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
            ui.heading("Wallhaven Browser");

            ui.horizontal(|ui| {
                if ui.button("Home").clicked() {
                    self.load_wallpapers("https://wallhaven.cc/", ctx);
                }
                if ui.button("Latest").clicked() {
                    self.load_wallpapers("https://wallhaven.cc/latest", ctx);
                }
                if ui.button("Top").clicked() {
                    self.load_wallpapers("https://wallhaven.cc/toplist", ctx);
                }
                if ui.button("Random").clicked() {
                    self.load_wallpapers("https://wallhaven.cc/random", ctx);
                }
                ui.text_edit_singleline(&mut self.search_query);
                if ui.button("Search").clicked() {
                    let search_url = format!("https://wallhaven.cc/search?q={}", self.search_query);
                    self.load_wallpapers(&search_url, ctx);
                }
            });

            // Add sliders for image size and columns
            ui.horizontal(|ui| {
                ui.label("Image Size:");
                ui.add(egui::Slider::new(&mut self.image_size, 100.0..=600.0).text("Size"));
            });

            ui.horizontal(|ui| {
                ui.label("Columns:");
                ui.add(egui::Slider::new(&mut self.columns, 1..=10).text("Cols"));
            });

            let available_width = ui.available_width();
            let image_width = available_width / self.columns as f32;
            let image_height = self.image_size;
            let image_size = egui::vec2(image_width, image_height);

            egui::ScrollArea::vertical().show(ui, |ui| {
                egui::Grid::new("wallpapers_grid").spacing([0.0, 0.0]).show(ui, |ui| {
                    for (i, (img_src, wallpaper_url)) in self.wallpapers.iter().enumerate() {
                        if i % self.columns == 0 {
                            ui.end_row();
                        }
                        if let Some(texture) = self.textures.get(img_src) {
                            if let Some(texture) = texture {
                                let image = egui::Image::new(texture);
                                if ui.add_sized(image_size, image).clicked() {
                                    if let Err(e) = open::that(wallpaper_url) {
                                        eprintln!("Failed to open URL: {}", e);
                                    }
                                }
                            }
                        } else {
                            // Skip if the texture is not ready yet
                            continue;
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

                let nodes_len = nodes.len();
                let (tx, rx) = mpsc::channel();

                thread::scope(|s| {
                    for (img_src, wallpaper_url) in nodes {
                        let tx = tx.clone();
                        let ctx = ctx.clone();
                        s.spawn(move |_| {
                            let texture = match load_image_from_url(&img_src, &ctx) {
                                Ok(texture) => Some(texture),
                                Err(_) => None,
                            };
                            tx.send((img_src, wallpaper_url, texture)).unwrap();
                        });
                    }
                }).unwrap();

                for (img_src, wallpaper_url, texture) in rx.iter().take(nodes_len) {
                    self.wallpapers.push((img_src.clone(), wallpaper_url));
                    self.textures.insert(img_src, texture);
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
