use dronegowski_utils::network::SimulationControllerNodeType;
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        // Personalizzazione dello stile per il pannello
        let original_visuals = ui.visuals().clone(); // Salviamo lo stile originale
        let mut custom_visuals = original_visuals.clone();
        custom_visuals.widgets.noninteractive.bg_fill = egui::Color32::from_rgb(94, 199, 113); // Verde #5EC771
        ui.set_visuals(custom_visuals);

        // Disegniamo il pannello con altezza aumentata
        egui::TopBottomPanel::bottom("bottom_panel")
            .exact_height(50.0) // Altezza maggiore
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(10.0); // Margine a sinistra
                    if ui.button("Spawn").clicked() {
                        // Logica per il pulsante Spawn
                    }
                });
            });

        ui.set_visuals(original_visuals); // Ripristiniamo lo stile originale
    }
}