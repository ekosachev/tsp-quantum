use eframe::{App, Frame, egui};

pub struct TspQuantumApp {}

impl App for TspQuantumApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut Frame) {}

    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut Frame) {}
}

impl TspQuantumApp {
    pub fn new() -> Self {
        Self {}
    }
}
