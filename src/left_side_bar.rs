use eframe::egui;
use egui::{Ui, Color32, Style, Visuals};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {

    pub fn left_side_panel(ui: &mut Ui){
        let mut active_button = 0; // Indice del pulsante attivo
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        for (i, label) in buttons.iter().enumerate() {
            let is_active = active_button == i;

            // Stile del bottone
            let button_color = if is_active {
                Color32::from_gray(200) // Grigio chiaro per bottone selezionato
            } else {
                Color32::WHITE // Bianco per i bottoni non selezionati
            };

            // Stile del testo
            let text_color = Color32::BLACK;

            // Disegna il bottone con stile personalizzato
            let response = ui.add(
                egui::Button::new(*label)
                    .fill(button_color) // Sfondo
                    .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
            );

            // Colora il testo
            ui.painter().text(
                response.rect.center(), // Posizione
                egui::Align2::CENTER_CENTER,
                label,
                egui::TextStyle::Button.resolve(ui.style()), // Stile testo
                text_color, // Colore testo
            );

            // Aggiorna il pulsante attivo
            if response.clicked() {
                active_button = i;
            }
        }
    }
}