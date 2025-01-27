use std::collections::HashMap;
use eframe::egui;
use eframe::egui::Key::M;
use eframe::egui::TextStyle;
use egui::{Color32, Ui, Id, Align2};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {


    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        // Definiamo i pulsanti
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        // Recuperiamo lo stato persistente del pulsante attivo
        let active_button_id = Id::new("active_button");
        let mut active_button = ui
            .data_mut(|d| d.get_persisted::<usize>(active_button_id))
            .unwrap_or(0);

        for (i, label) in buttons.iter().enumerate() {
            let is_active = active_button == i;

            // Stile del bottone
            let button_color = if is_active {
                Color32::from_gray(300) // Grigio chiaro per bottone selezionato
            } else {
                Color32::WHITE // Bianco per i bottoni non selezionati
            };

            // Calcolo della dimensione del testo
            let button_size = ui
                .fonts(|fonts| {
                    fonts
                        .layout_no_wrap(
                            label.to_string(),
                            TextStyle::Button.resolve(ui.style()),
                            Color32::BLACK, // Colore del testo richiesto dalla nuova API
                        )
                        .rect
                        .size()
                });

            // Disegno del bottone con dimensione calcolata
            let response = ui.add_sized(
                [button_size.x + 20.0, button_size.y + 10.0], // Aggiunta di padding
                egui::Button::new(" ") // Bottone vuoto
                    .fill(button_color) // Sfondo
                    .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
            );

            // Disegno del testo sopra il bottone
            ui.painter().text(
                response.rect.center(), // Centro del pulsante
                Align2::CENTER_CENTER,  // Allineamento centrato
                label,                  // Testo del pulsante
                TextStyle::Button.resolve(ui.style()), // Stile del testo
                Color32::DARK_GRAY,         // Colore del testo
            );

            // Aggiorniamo lo stato se il pulsante è cliccato
            if response.clicked() {
                active_button = i;
                ui.data_mut(|d| d.insert_persisted(active_button_id, active_button));
            }
        }
    }


}