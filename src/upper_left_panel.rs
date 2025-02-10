use dronegowski_utils::network::{SimulationControllerNode};
use eframe::egui;
use eframe::egui::{Color32, Direction, Layout, RichText, TextBuffer, TextEdit};
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn upper_left_panel_default(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new("SIMULATION COMMANDS").size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.add_space(5.);
                // Imposta la dimensione del campo di testo con `.min_size(width, height)`
            let response = ui.add(
                TextEdit::singleline(&mut self.panel.upper_left_panel.spawn_pdr)
                    .min_size(egui::Vec2::new(200.0, 40.0)) // Larghezza: 300, Altezza: 50
                    .font(egui::FontId::new(30.0, egui::FontFamily::Proportional)) // Grandezza del testo: 20.0
                    .frame(true) // Abilita il frame intorno al campo
                    .background_color(Color32::WHITE), // Colore di sfondo
            );

            // Se il campo di testo è vuoto e non ha il focus, disegna il placeholder con testo più grande
            if self.panel.upper_left_panel.spawn_pdr.is_empty() && !response.has_focus() {
                ui.painter().text(
                    response.rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Set PDR",
                    egui::FontId::new(30.0, egui::FontFamily::Proportional), // Grandezza del placeholder: 20.0
                    ui.style().visuals.text_color().gamma_multiply(0.8), // Colore grigio per il placeholder
                );
            }
        });

        ui.add_space(10.);
        ui.horizontal(|ui|{
            ui.add_space(220.);
            if ui.add(egui::Button::new(

                RichText::new("Spawn").color(Color32::WHITE)
                    .size(15.0)).fill(Color32::from_rgb(0, 119, 182)).min_size(egui::Vec2::new(60.0, 30.0))).clicked() && self.panel.upper_left_panel.spawn_pdr != "" {
                let mut pdr = self.panel.upper_left_panel.spawn_pdr.clone().parse::<f32>();
                if pdr.is_ok() {
                    if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                        self.spawn(pdr.clone().unwrap());
                        self.panel.upper_left_panel.spawn_pdr = "".to_string();
                    }
                }
            }
        });


    }

    pub fn upper_left_panel_drone(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("DRONE {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(50.);

        ui.horizontal(|ui| {
            ui.add_space(5.);
            // Imposta la dimensione del campo di testo con `.min_size(width, height)`
            let response = ui.add(
                TextEdit::singleline(&mut self.panel.upper_left_panel.change_pdr)
                    .min_size(egui::Vec2::new(200.0, 40.0)) // Larghezza: 300, Altezza: 50
                    .font(egui::FontId::new(30.0, egui::FontFamily::Proportional)) // Grandezza del testo: 20.0
                    .frame(true) // Abilita il frame intorno al campo
                    .background_color(Color32::WHITE), // Colore di sfondo
            );

            // Se il campo di testo è vuoto e non ha il focus, disegna il placeholder con testo più grande
            if self.panel.upper_left_panel.change_pdr.is_empty() && !response.has_focus() {
                ui.painter().text(
                    response.rect.left_center(),
                    egui::Align2::LEFT_CENTER,
                    "Set PDR",
                    egui::FontId::new(30.0, egui::FontFamily::Proportional), // Grandezza del placeholder: 20.0
                    ui.style().visuals.text_color().gamma_multiply(0.8), // Colore grigio per il placeholder
                );
            }
        });

        ui.add_space(10.);
        ui.horizontal(|ui|{
            ui.add_space(220.);
            if ui.add(egui::Button::new(

                RichText::new("Change").color(Color32::WHITE)
                    .size(15.0)).fill(Color32::from_rgb(0, 119, 182)).min_size(egui::Vec2::new(60.0, 30.0))).clicked() && self.panel.upper_left_panel.spawn_pdr != "" {
                let mut pdr = self.panel.upper_left_panel.change_pdr.clone().parse::<f32>();
                if pdr.is_ok() {
                    if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                        self.set_pdr(pdr.clone().unwrap());
                        self.panel.upper_left_panel.change_pdr = "".to_string();
                    }
                }
            }
        });
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.add_space(20.);
            if ui.add(egui::Button::new(
                RichText::new("Add Sender") // Add an emoji or icon
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::GREEN).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.add_sender = true;
            }
            //Color32::from_rgb(0, 119, 182)
            //Color32::from_rgb(0, 168, 150)
            //Color32::from_rgb(244, 162, 97)
            //Color32::from_rgb(234, 234, 234)
            //Color32::from_rgb(136, 14, 79)
            ui.add_space(20.);

            if ui.add(egui::Button::new(
                RichText::new("Remove Sender") // Add an emoji or icon
                    .color(Color32::WHITE)
                    .size(16.0)).fill(Color32::RED).min_size(egui::Vec2::new(110.0, 50.0))).clicked(){
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);
        });
        ui.add_space(20.);
        if ui.button("Crash").clicked() {
            self.panel.upper_left_panel.crash = true;
        }
    }

    pub fn upper_left_panel_client(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("CLIENT {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(20.);

        if ui.button("Add Sender").clicked(){
            self.panel.upper_left_panel.add_sender = true;
        }
        ui.add_space(10.);

        if ui.button("Remove Sender").clicked(){
            self.panel.upper_left_panel.remove_sender = true;
        }
    }

    pub fn upper_left_panel_server(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("SERVER {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(20.);

        if ui.button("Add Sender").clicked(){
            self.panel.upper_left_panel.add_sender = true;
        }
        ui.add_space(10.);

        if ui.button("Remove Sender").clicked(){
            self.panel.upper_left_panel.remove_sender = true;
        }
    }
}



