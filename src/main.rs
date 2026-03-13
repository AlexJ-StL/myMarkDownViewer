#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use rfd::FileDialog;
use std::env;
use std::fs;

struct MdViewer {
    markdown_text: String,
    file_path: Option<String>,
    cache: CommonMarkCache,
}

impl MdViewer {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            markdown_text: String::from("# Welcome to MdViewer\n\nClick **Open File...** to load a Markdown file."),
            file_path: None,
            cache: CommonMarkCache::default(),
        };

        // Check for CLI arguments
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let path = &args[1];
            if let Ok(text) = fs::read_to_string(path) {
                app.markdown_text = text;
                app.file_path = Some(path.clone());
            } else {
                app.markdown_text = format!("# Error\n\nCould not read file: `{}`", path);
            }
        }

        app
    }

    fn open_file(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("Markdown", &["md", "markdown", "txt"])
            .pick_file()
        {
            if let Ok(text) = fs::read_to_string(&path) {
                self.markdown_text = text;
                self.file_path = Some(path.to_string_lossy().into_owned());
            } else {
                self.markdown_text = format!("# Error\n\nCould not read file.");
            }
        }
    }
}

impl eframe::App for MdViewer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("📂 Open File...").clicked() {
                    self.open_file();
                }
                if let Some(path) = &self.file_path {
                    ui.label(format!("Viewing: {}", path));
                } else {
                    ui.label("No file loaded.");
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                CommonMarkViewer::new().show(
                    ui,
                    &mut self.cache,
                    &self.markdown_text,
                );
            });
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("Markdown Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Lightweight Markdown Viewer",
        options,
        Box::new(|cc| Ok(Box::new(MdViewer::new(cc)))),
    )
}
