use std::collections::HashMap;
use eframe::egui;
use eframe::egui::Key::M;
use egui::{Ui, Color32, Style, Visuals};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {

    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        let mut active_button = 0; // Indice del pulsante attivo

        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        let mut buttons = HashMap::new();
        buttons.insert("Visualizzazione network", true);
        buttons.insert("Notifiche SC", false);
        buttons.insert("Lista nodi network", false);


        for mut button in &buttons{

            // Stile del bottone
            let button_color = if button.1 {
                Color32::from_gray(200) // Grigio chiaro per bottone selezionato
            } else {
                Color32::WHITE // Bianco per i bottoni non selezionati
            };

            // Stile del testo
            let text_color = Color32::BLACK;

            // Disegna il bottone con stile personalizzato
            let response = ui.add(
                egui::Button::new(button.0)
                    .fill(button_color) // Sfondo
                    .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
            );

            // Colora il testo
            ui.painter().text(
                response.rect.center(), // Posizione
                egui::Align2::CENTER_CENTER,
                button.0,
                egui::TextStyle::Button.resolve(ui.style()), // Stile testo
                text_color, // Colore testo
            );

            // Aggiorna il pulsante attivo
            if response.clicked() {
                if !button.1{
                    for mut b in &buttons{
                        b.1 = &false;
                    }
                    button.1 = &true;
                }
            }
        }

    }
}