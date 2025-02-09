use eframe::egui;
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn right_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("NODE LIST".to_string());
        for elem in &self.nodi{

        }
    }
}