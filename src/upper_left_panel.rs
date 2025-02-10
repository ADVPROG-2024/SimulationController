use dronegowski_utils::network::{SimulationControllerNode};
use eframe::egui;
use eframe::egui::{Color32, Direction, Layout, RichText};
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
            let pdr = self.panel.upper_left_panel.spawn_pdr.clone().parse::<f32>().unwrap();
            if pdr <= 1. && pdr >= 0. {
                self.spawn(pdr);
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
            if ui.button("Add Sender").clicked() {
                self.panel.upper_left_panel.add_sender = true;
            }
            ui.add_space(10.);

            if ui.button("Remove Sender").clicked() {
                self.panel.upper_left_panel.remove_sender = true;
            }
            ui.add_space(10.);

            ui.horizontal(|ui|{
                if ui.button("Set PDR").clicked() && self.panel.upper_left_panel.change_pdr != "" {
                    let pdr = self.panel.upper_left_panel.change_pdr.clone().parse::<f32>().unwrap();
                    if pdr <= 1. && pdr >= 0. {
                        self.set_pdr(pdr);
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



