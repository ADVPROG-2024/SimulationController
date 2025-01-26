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

            // Configurazione temporanea dello stile per il testo e il bottone
            let text_color = Color32::BLACK;
            let visuals = ui.style_mut().visuals.clone();

            ui.style_mut().visuals.widgets.inactive.bg_fill = button_color; // Sfondo del bottone
            ui.style_mut().visuals.widgets.inactive.fg_stroke.color = text_color; // Colore del testo

            if ui.button(*label).clicked() {
                active_button = i; // Cambia il bottone attivo
            }

            // Ripristina lo stile originale dopo aver disegnato il bottone
            ui.style_mut().visuals = visuals;
        }
    }
}
