use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        // Aumentiamo l'altezza e impostiamo il colore di sfondo
        ui.add_space(20.0); // Altezza extra sopra
        ui.painter()
            .rect_filled(ui.max_rect(), 0.0, egui::Color32::from_rgb(94, 199, 113)); // Colore verde #5EC771

        ui.horizontal(|ui| {
            ui.add_space(10.0); // Margine a sinistra
            if ui.button("Spawn").clicked() {
                // Logica per il pulsante Spawn
            }
        });
        ui.add_space(20.0); // Altezza extra sotto
    }
}