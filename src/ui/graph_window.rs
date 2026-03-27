use eframe::egui::{self};

#[derive(Default)]
pub struct GraphWindow {
    nodes: Vec<egui::Pos2>,
    screen_origin: egui::Pos2,
    hovered_node: Option<usize>,
    dragged_node: Option<usize>,
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
        self.update_hovered_node(response);
        self.create_node(response);
        self.move_node(response);
    }

    fn paint_nodes(&self, painter: &egui::Painter) {
        self.nodes.iter().for_each(|n| {
            let node_color = self
                .hovered_node
                .map(|i| {
                    if self.nodes[i] == *n {
                        egui::Color32::LIGHT_GREEN
                    } else {
                        egui::Color32::GREEN
                    }
                })
                .unwrap_or(egui::Color32::GREEN);
            painter.circle_filled(self.screen_origin + n.to_vec2(), 5.0, node_color);
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

    fn create_node(&mut self, response: &egui::Response) {
        if let Some(interact_pos) = response.interact_pointer_pos()
            && response.clicked()
        {
            self.nodes.push(interact_pos - self.screen_origin.to_vec2());
        }
    }

    fn update_hovered_node(&mut self, response: &egui::Response) {
        if let Some(hover_pos) = response.hover_pos() {
            self.hovered_node = self
                .nodes
                .iter()
                .position(|n| hover_pos.distance(self.screen_origin + (*n).to_vec2()) < 10.0);
        }
    }

    fn move_node(&mut self, response: &egui::Response) {
        if self.dragged_node.is_none()
            && response.drag_started()
            && let Some(i) = self.hovered_node
        {
            self.dragged_node = Some(i);
        }

        if self.dragged_node.is_some() && response.drag_stopped() {
            self.dragged_node = None;
        }

        if let Some(i) = self.dragged_node
            && response.dragged()
        {
            self.nodes[i] += response.drag_delta();
        }
    }
}
