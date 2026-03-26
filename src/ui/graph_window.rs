use eframe::egui::{self};

#[derive(Default)]
pub struct GraphWindow {
    nodes: Vec<egui::Pos2>,
    screen_origin: egui::Pos2,
}

impl GraphWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let (response, painter) = ui.allocate_painter(
            ui.available_size_before_wrap(),
            egui::Sense::click_and_drag(),
        );
        self.screen_origin = painter.clip_rect().left_top();
        self.logic(&response);
        self.paint_nodes(&painter);
    }

    pub fn logic(&mut self, response: &egui::Response) {
        if let Some(interact_pos) = response.interact_pointer_pos()
            && response.clicked()
        {
            self.nodes.push(interact_pos - self.screen_origin.to_vec2())
        }
    }

    fn paint_nodes(&self, painter: &egui::Painter) {
        self.nodes.iter().for_each(|n| {
            painter.circle_filled(self.screen_origin + n.to_vec2(), 5.0, egui::Color32::GREEN);
        });
    }
}
