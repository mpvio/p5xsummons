use crate::views::p5xui::P5XUI;
use egui::ViewportBuilder;

pub mod models;
pub mod controllers;
pub mod views;

// TODO
/*
* if a database is found -> replace item signifiers (aas) with item names
*/
#[tokio::main]
async fn main() {
    let app = P5XUI::new().await;
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([300.0, 70.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Persona 5 X Contract Tracker",
        options,
        Box::new(|_cc| Ok(Box::new(app)))
    ).unwrap();
}
