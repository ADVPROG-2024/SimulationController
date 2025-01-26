use eframe::egui;
use eframe::egui::{Ui, Color32};
use crate::{simulation_controller, DronegowskiSimulationController};

pub struct MyApp {
    active_button: usize, // Indice del bottone attivo
}

impl MyApp {
    pub fn new() -> Self {
        Self { active_button: 0 } // Il primo bottone è attivo di default
    }

    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        for (i, label) in buttons.iter().enumerate() {
            let is_active = self.active_button == i;

            // Colore del bottone in base all'attività
            let button_color = if is_active {
                Color32::from_rgb(100, 150, 255) // Blu chiaro per attivo
            } else {
                Color32::from_rgb(220, 220, 220) // Grigio chiaro per inattivo
            };

            if ui.add(egui::Button::new(label).fill(button_color)).clicked() {
                self.active_button = i; // Cambia il bottone attivo
            }
        }
    }
}
