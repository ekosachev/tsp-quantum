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
        self.paint_complete_edges(&painter);
        self.paint_nodes(&painter);
    }

    fn logic(&mut self, response: &egui::Response) {
        if let Some(interact_pos) = response.interact_pointer_pos()
            && response.clicked()
        {
            self.nodes.push(interact_pos - self.screen_origin.to_vec2());
        }
    }

    fn paint_nodes(&self, painter: &egui::Painter) {
        self.nodes.iter().for_each(|n| {
            painter.circle_filled(self.screen_origin + n.to_vec2(), 5.0, egui::Color32::GREEN);
        });
    }

    fn paint_complete_edges(&self, painter: &egui::Painter) {
        self.nodes.iter().enumerate().for_each(|(i, u)| {
            self.nodes.iter().skip(i + 1).for_each(|v| {
                painter.line_segment(
                    [
                        self.screen_origin + (*u).to_vec2(),
                        self.screen_origin + (*v).to_vec2(),
                    ],
                    egui::Stroke::new(1.0, egui::Color32::DARK_GRAY),
                );
            });
        });
    }
}
