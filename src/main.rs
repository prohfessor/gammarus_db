mod db;
mod app;
mod views;

use eframe::egui;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "База данных Eucarinogammarus",
        options,
        Box::new(|cc| Box::new(app::EucarinogammarusApp::new(cc))),
    )?;
    
    Ok(())
}
