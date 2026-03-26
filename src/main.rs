#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod ui;

use app::TspQuantumApp;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "TSP Quantum",
        native_options,
        Box::new(|_cc| Ok(Box::new(TspQuantumApp::new()))),
    );
}
