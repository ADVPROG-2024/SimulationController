use eframe::egui;
use egui::{Ui, Color32, Style, Visuals};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {

    pub fn left_side_panel(ui: &mut Ui) {
        let mut active_button = 0; // Indice del pulsante attivo
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        ui.vertical(|ui| {
            for (i, label) in buttons.iter().enumerate() {
                let is_active = active_button == i;

                // Definizione dei colori
                let button_color = if is_active {
                    Color32::from_gray(200) // Grigio chiaro se attivo
                } else {
                    Color32::WHITE // Bianco se non attivo
                };

                let text_color = Color32::BLACK;

                // Disegna il bottone con stile personalizzato
                if ui
                    .add(
                        egui::Button::new(*label)
                            .fill(button_color) // Sfondo
                            .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
                    )
                    .clicked()
                {
                    // Aggiorna il pulsante attivo
                    active_button = i;
                }

                // Disegna il testo sopra il bottone
                ui.painter().text(
                    ui.cursor().min, // Posizione del testo
                    egui::Align2::CENTER_CENTER,
                    label,
                    egui::TextStyle::Button.resolve(ui.style()),
                    text_color,
                );
            }
        });
    }
}