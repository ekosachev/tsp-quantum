use crate::{solver::tsp_qubo::QUBO, ui};
use eframe::{App, Frame, egui};

#[derive(Default)]
pub struct TspQuantumApp {
    graph_window: ui::GraphWindow,
    graph_window_response: ui::GraphWindowResponse,
    qubo: QUBO,
    qubo_window: ui::QuboWindow,
}

impl App for TspQuantumApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut Frame) {
        if self.graph_window_response.updated {
            let nodes = self.graph_window.nodes.clone();
            self.update_qubo(nodes);
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Window::new("Graph").show(ui.ctx(), |ui| {
            self.graph_window_response = self.graph_window.ui(ui)
        });
        egui::Window::new("QUBO").show(ui.ctx(), |ui| self.qubo_window.ui(ui));
    }
}

impl TspQuantumApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn update_qubo(&mut self, nodes: Vec<egui::Pos2>) {
        self.qubo.num_cities = nodes.len();
        self.qubo.state = vec![0.0; self.qubo.num_variables().pow(2)];

        self.qubo_window.n = self.qubo.num_variables();
        self.qubo_window.state = self.qubo.state.iter().map(|v| f64::from(*v)).collect();
    }
}
