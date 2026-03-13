#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use rfd::FileDialog;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

struct MdViewer {
    markdown_text: String,
    file_path: Option<String>,
    base_dir: Option<PathBuf>,
    cache: CommonMarkCache,
}

impl MdViewer {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Install image loaders
        egui_extras::install_image_loaders(&cc.egui_ctx);

        let mut app = Self {
            markdown_text: String::from(
                "# Welcome to MdViewer\n\nClick **Open File...** to load a Markdown file.",
            ),
            file_path: None,
            base_dir: None,
            cache: CommonMarkCache::default(),
        };

        // Check for CLI arguments
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let path_str = &args[1];
            if let Ok(text) = fs::read_to_string(path_str) {
                app.markdown_text = text;
                app.file_path = Some(path_str.clone());
                app.base_dir = Path::new(path_str).parent().map(|p| p.to_path_buf());
            } else {
                app.markdown_text = format!("# Error\n\nCould not read file: `{}`", path_str);
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
                self.base_dir = path.parent().map(|p| p.to_path_buf());
            } else {
                self.markdown_text = format!("# Error\n\nCould not read file.");
            }
        }
    }

    /// Pre-process markdown text to resolve relative image paths
    fn processed_markdown(&self) -> String {
        let Some(base_dir) = &self.base_dir else {
            return self.markdown_text.clone();
        };

        let base_dir_str = base_dir.to_string_lossy().replace('\\', "/");
        let mut result = self.markdown_text.clone();

        // Simple regex-like replacement for Markdown images: ![alt](path)
        // We look for patterns like ](path) and check if 'path' is relative.
        // This is a heuristic but matches the User's example: ![image.png](Figures/34b630e8-8a21-4e1c-8cbe-7987eb66f07e.png)

        let mut offset = 0;
        let original = self.markdown_text.clone();

        // Find all occurrences of "]( "
        for (start_match, _) in original.match_indices("](") {
            let path_start = start_match + 2;
            if let Some(path_end) = original[path_start..].find(')') {
                let path_end = path_start + path_end;
                let path = &original[path_start..path_end];

                // If it's not a URL and not absolute
                if !path.starts_with("http") && !Path::new(path).is_absolute() {
                    let absolute_path = format!("file:///{}/{}", base_dir_str, path);
                    result
                        .replace_range((path_start + offset)..(path_end + offset), &absolute_path);
                    offset += absolute_path.len() - path.len();
                }
            }
        }

        result
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
                let markdown = self.processed_markdown();
                CommonMarkViewer::new().show(ui, &mut self.cache, &markdown);
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
