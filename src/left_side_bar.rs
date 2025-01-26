use std::collections::HashMap;
use eframe::egui;
use eframe::egui::Key::M;
use egui::{Color32, Ui, Id};
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
                Color32::from_gray(200) // Grigio chiaro per bottone selezionato
            } else {
                Color32::WHITE // Bianco per i bottoni non selezionati
            };

            // Disegniamo il bottone
            let response = ui.add(
                egui::Button::new(*label)
                    .fill(button_color) // Sfondo
                    .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
            );

            // Aggiorniamo lo stato se il pulsante è cliccato
            if response.clicked() {
                active_button = i;
                ui.data_mut(|d| d.insert_persisted(active_button_id, active_button));
            }
        }
    }

}