use eframe::egui;

#[derive(Default)]
pub struct GraphWindow {
    pub _nodes: Vec<egui::Pos2>,
}

impl GraphWindow {
    pub fn ui(&mut self, _ui: &mut egui::Ui) {}
}
