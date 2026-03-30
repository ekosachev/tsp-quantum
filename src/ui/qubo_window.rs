use eframe::egui::{self, Widget};
use egui_plot::{AxisHints, HPlacement, Heatmap, Plot, VPlacement};

#[derive(Clone, Copy)]
pub struct QuboWeightControls {
    pub visit_once: f64,
    pub one_at_a_time: f64,
    pub distance: f64,
}

impl Default for QuboWeightControls {
    fn default() -> Self {
        Self {
            visit_once: 1.0,
            one_at_a_time: 1.0,
            distance: 1.0,
        }
    }
}

#[derive(Default)]
pub struct QuboWindow {
    pub state: Vec<f64>,
    pub n_cities: usize,
    pub weight_controls: QuboWeightControls,
}

#[derive(Default)]
pub struct QuboWindowResponse {
    pub update_weights: bool,
}

impl QuboWindow {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> QuboWindowResponse {
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

        let visit_once_response =
            egui::Slider::new(&mut self.weight_controls.visit_once, 0.0..=10.0)
                .text("Visit each city only once")
                .ui(ui);

        let one_at_a_time_response =
            egui::Slider::new(&mut self.weight_controls.one_at_a_time, 0.0..=10.0)
                .text("Visit only one city at a time")
                .ui(ui);

        let distance_response = egui::Slider::new(&mut self.weight_controls.distance, 0.0..=10.0)
            .text("Go to the closest city")
            .ui(ui);

        QuboWindowResponse {
            update_weights: visit_once_response.changed()
                || one_at_a_time_response.changed()
                || distance_response.changed(),
        }
    }
}
