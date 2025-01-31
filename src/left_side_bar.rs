use eframe::egui;
use eframe::egui::TextStyle;
use egui::{Color32, Ui, Id, Align2};
use crate::{DronegowskiSimulationController};
use crate::sc_utils::LeftButton;

impl DronegowskiSimulationController {

    pub fn left_side_panel(&mut self, ui: &mut Ui) {
        // Definiamo i pulsanti
        let buttons = vec![
            "Visualizzazione network",
            "Notifiche SC",
            "Lista nodi network",
        ];

        // Recuperiamo lo stato persistente del pulsante attivo
        for label in buttons.iter(){
            let is_active = **label == self.panel.left_panel.left_button;

            // Stile del bottone
            let button_color = if is_active {
                Color32::from_gray(200) // Grigio chiaro per bottone selezionato
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
                self.panel.left_panel.left_button = label.to_string();
                if *label == "Visualizzazione network" { self.panel.left_panel.active_left_button = LeftButton::Network}
                else if *label == "Notifiche SC" { self.panel.left_panel.active_left_button = LeftButton::Notifiche}
                else {self.panel.left_panel.active_left_button = LeftButton::Lista}
            }
        }
    }


}