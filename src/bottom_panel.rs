use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        // Specifica l'altezza del pannello
        let panel_rect = ui.max_rect();
        ui.painter()
            .rect_filled(panel_rect, 0.0, egui::Color32::from_rgb(94, 199, 113)); // Verde #5EC771

        // Contenuto del pannello
        ui.horizontal(|ui| {
            ui.add_space(10.0); // Margine a sinistra
            if ui.button("Spawn").clicked() {
                // Logica per il pulsante Spawn
            }
        });
        // Non è necessario chiamare `advance_cursor` in quanto il layout è gestito automaticamente.
    }
}