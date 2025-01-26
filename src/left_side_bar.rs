use eframe::egui;
use egui::{Ui, Color32, Style, Visuals};
use crate::{simulation_controller, DronegowskiSimulationController};

impl DronegowskiSimulationController {

    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        // Stato locale per ogni bottone (inizialmente tutti non selezionati)
        let mut button_states = vec![false, false, false]; // Ogni elemento rappresenta lo stato di un bottone
        let labels = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        ui.vertical(|ui| {
            for (i, label) in labels.iter().enumerate() {
                // Controlla lo stato del bottone
                let is_active = button_states[i];

                // Colori in base allo stato
                let button_color = if is_active {
                    Color32::from_gray(200) // Grigio chiaro se attivo
                } else {
                    Color32::WHITE // Bianco se non attivo
                };

                let text_color = Color32::BLACK;

                // Crea il bottone
                if ui
                    .add(
                        egui::Button::new(*label)
                            .fill(button_color)
                            .stroke(egui::Stroke::new(1.0, Color32::BLACK)), // Bordo
                    )
                    .clicked()
                {
                    // Aggiorna gli stati: solo il bottone cliccato diventa attivo
                    for state in button_states.iter_mut() {
                        *state = false; // Resetta tutti gli altri bottoni
                    }
                    button_states[i] = true; // Attiva solo il bottone cliccato
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
    }}