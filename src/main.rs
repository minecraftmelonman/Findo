#![windows_subsystem = "windows"]

mod search;
mod ui;

use crate::ui::FindoApp;
use eframe::egui;

fn main() -> eframe::Result<()> {
    // whimsy font
    println!(
        r"


   ,d8888b  d8,                d8b    
   88P'     `8P                 88P    
d888888P                      d88  
  ?88'      88b  88bd88b  d888888   d8888b 
  88P       88P  88P' ?8bd8P' ?88  d8P' ?88
 d88       d88  d88   88P88b  ,88b 88b  d88
d88'      d88' d88'   88b`?88P'`88b`?8888P'

Starting application..."
    );

    let icon_bytes: &[u8] = include_bytes!("assets/Findo.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();

    let icon_data = egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    };

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Findo File Search",
        options,
        Box::new(|_cc| Box::new(FindoApp::default()) as Box<dyn eframe::App>),
    )
}