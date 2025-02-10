use dronegowski_utils::network::{SimulationControllerNode};
use eframe::egui;
use eframe::egui::{Color32, Direction, Layout, RichText, TextBuffer};
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn upper_left_panel_default(&mut self, ui: &mut egui::Ui) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new("SIMULATION COMMANDS").size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(20.);

        if ui.button("Spawn").clicked() && self.panel.upper_left_panel.spawn_pdr != "" {
            let mut pdr = self.panel.upper_left_panel.spawn_pdr.clone().parse::<f32>();
            if pdr.is_ok() {
                if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                    self.spawn(pdr.clone().unwrap());
                    self.panel.upper_left_panel.spawn_pdr = "".to_string();
                }
            }
        }
        ui.text_edit_singleline(&mut self.panel.upper_left_panel.spawn_pdr);

        ui.add_space(10.);

        if ui.button("Send").clicked(){
            self.send_packet_test();
        }
    }

    pub fn upper_left_panel_drone(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.add_space(20.);
        ui.horizontal(|ui|{
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.heading(RichText::new(format!("DRONE {} COMMANDS", node.node_id)).size(25.0).color(Color32::BLACK));
            });
        });
        ui.add_space(20.);

        ui.vertical(|ui| {
            if ui.add(egui::Button::new(
                egui::RichText::new("📤 Add Sender") // Add an emoji or icon
                    .color(egui::Color32::WHITE)
                    .size(16.0)).fill(egui::Color32::from_rgb(30, 30, 30)).min_size(egui::Vec2::new(120.0, 50.0))).clicked(){

                          self.panel.upper_left_panel.add_sender = true;
            }
            //Color32::from_rgb(0, 119, 182)
            //Color32::from_rgb(0, 168, 150)
            //Color32::from_rgb(244, 162, 97)
            //Color32::from_rgb(234, 234, 234)
            //Color32::from_rgb(136, 14, 79)
            ui.add_space(10.);

            if ui.button("Remove Sender").clicked() {
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);

            ui.horizontal(|ui|{
                if ui.button("Set PDR").clicked() && self.panel.upper_left_panel.change_pdr != "" {
                    let mut pdr = self.panel.upper_left_panel.change_pdr.clone().parse::<f32>();
                    if pdr.is_ok() {
                        if pdr.clone().unwrap() <= 1. && pdr.clone().unwrap() >= 0. {
                            self.set_pdr(pdr.clone().unwrap());
                            self.panel.upper_left_panel.change_pdr = "".to_string();
                        }
                    }
                }

                ui.text_edit_singleline(&mut self.panel.upper_left_panel.change_pdr);
            });
            ui.add_space(10.);

            if ui.button("Crash").clicked() {
                self.panel.upper_left_panel.crash = true;
            }
        });
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



