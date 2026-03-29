use eframe::egui;
use egui_plot::{AxisHints, HPlacement, Heatmap, Plot, VPlacement};

#[derive(Default)]
pub struct QuboWindow {
    pub state: Vec<f64>,
    pub n_cities: usize,
}

impl QuboWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let heatmap = Heatmap::new(self.state.clone(), self.n_cities.pow(2));

        let x_axes = vec![
            AxisHints::new_x()
                .label("City")
                .placement(VPlacement::Top)
                .formatter(|tick, _range| {
                    ((tick.value as usize) / self.n_cities.max(1)).to_string()
                })
                .label_spacing(1.0f32..=1.0f32),
            AxisHints::new_x()
                .label("Position")
                .placement(VPlacement::Bottom)
                .formatter(|tick, _range| {
                    ((tick.value as usize) % self.n_cities.max(1)).to_string()
                })
                .label_spacing(1.0f32..=1.0f32),
        ];

        let y_axes = vec![
            AxisHints::new_y()
                .label("City")
                .placement(HPlacement::Left)
                .formatter(|tick, _range| {
                    ((tick.value as usize) / self.n_cities.max(1)).to_string()
                })
                .label_spacing(1.0f32..=1.0f32),
            AxisHints::new_y()
                .placement(HPlacement::Right)
                .label("Position")
                .formatter(|tick, _range| {
                    ((tick.value as usize) % self.n_cities.max(1)).to_string()
                })
                .label_spacing(1.0f32..=1.0f32),
        ];

        Plot::new("qubo_plot")
            .allow_zoom(false)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .view_aspect(1.0)
            .show_grid(false)
            .show_crosshair(false)
            .invert_y(true)
            .custom_x_axes(x_axes)
            .custom_y_axes(y_axes)
            .show(ui, |plot_ui| plot_ui.heatmap(heatmap));
    }
}
