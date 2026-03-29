use eframe::egui;
use egui_plot::{Heatmap, Plot};

#[derive(Default)]
pub struct QuboWindow {
    pub state: Vec<f64>,
    pub n: usize,
}

impl QuboWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let heatmap = Heatmap::new(self.state.clone(), self.n);

        Plot::new("qubo_plot")
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .view_aspect(1.0)
            .show_grid(false)
            .show_crosshair(false)
            .show(ui, |plot_ui| plot_ui.heatmap(heatmap));
    }
}
