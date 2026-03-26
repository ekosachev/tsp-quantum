use crate::ui;
use eframe::{App, Frame, egui};

#[derive(Default)]
pub struct TspQuantumApp {
    graph_window: ui::GraphWindow,
}

impl App for TspQuantumApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Window::new("Graph").show(ui.ctx(), |ui| self.graph_window.ui(ui));
    }
}

impl TspQuantumApp {
    pub fn new() -> Self {
        Self::default()
    }
}
