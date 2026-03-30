use crate::{
    solver::{ising_model::IsingModel, tsp_qubo::QUBO},
    ui::{self, QuboWeightControls},
};
use eframe::{App, Frame, egui};

#[derive(Default)]
pub struct TspQuantumApp {
    graph_window: ui::GraphWindow,
    graph_window_response: ui::GraphWindowResponse,
    qubo: QUBO,
    qubo_window: ui::QuboWindow,
    qubo_window_response: ui::QuboWindowResponse,
    ising_window: ui::IsingWindow,
    ising_model: IsingModel,
}

impl App for TspQuantumApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut Frame) {
        if self.graph_window_response.updated {
            let nodes = self.graph_window.nodes.clone();
            self.update_qubo(nodes);
        }

        if self.qubo_window_response.update_weights {
            self.update_qubo_weights(self.qubo_window.weight_controls);
            self.update_ising_model();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut Frame) {
        egui::Window::new("Graph").show(ui.ctx(), |ui| {
            self.graph_window_response = self.graph_window.ui(ui)
        });
        egui::Window::new("QUBO").show(ui.ctx(), |ui| {
            self.qubo_window_response = self.qubo_window.ui(ui)
        });
        egui::Window::new("Ising").show(ui.ctx(), |ui| self.ising_window.ui(ui));
    }
}

impl TspQuantumApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn update_qubo(&mut self, nodes: Vec<egui::Pos2>) {
        self.qubo.num_cities = nodes.len();
        self.qubo
            .update_weights(self.graph_window.distance_matrix());

        self.qubo_window.n_cities = self.qubo.num_cities;
        self.qubo_window.state = self.qubo.state.clone();
    }

    fn update_qubo_weights(&mut self, weight_controls: QuboWeightControls) {
        self.qubo.weights.visit_once = weight_controls.visit_once;
        self.qubo.weights.one_city_in_position = weight_controls.one_at_a_time;
        self.qubo.weights.distance = weight_controls.distance;

        self.qubo
            .update_weights(self.graph_window.distance_matrix());
        self.qubo_window.state = self.qubo.state.clone();
    }

    fn update_ising_model(&mut self) {
        self.ising_model
            .load_qubo(self.qubo.num_variables(), &self.qubo.state);
    }
}
