use eframe::egui;
use egui_plot::{Heatmap, Plot};

#[derive(Default)]
pub struct QuboWindow {}

impl QuboWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let heatmap = Heatmap::new(vec![0.0, 0.25, 0.5, 0.75], 2);

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
