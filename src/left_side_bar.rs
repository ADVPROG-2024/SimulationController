use eframe::egui;
use egui::{Ui, Color32, Style, Visuals};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        let mut active_button = 0;
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        for (i, label) in buttons.iter().enumerate() {
            let is_active = active_button == i;

            // Colore dello sfondo del bottone
            let button_color = if is_active {
                Color32::from_gray(200) // Grigio chiaro per il bottone selezionato
            } else {
                Color32::WHITE // Bianco per i bottoni non selezionati
            };

            // Colore del testo: Nero
            let text_color = Color32::BLACK;

            if ui
                .add(
                    egui::Button::new(*label)
                        .fill(button_color)
                        .text_color(text_color),
                )
                .clicked()
            {
                active_button = i; // Cambia il bottone attivo
            }
        }
    }}
