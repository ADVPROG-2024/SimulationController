use dronegowski_utils::network::{SimulationControllerNode};
use eframe::egui;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn upper_left_panel_default(&mut self, ui: &mut egui::Ui) {
        ui.heading("SIMULATION CONTROLLER".to_string());
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
        ui.heading("Drone ".to_owned() + &*node.node_id.to_string());
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
        ui.heading("Client ".to_owned() + &*node.node_id.to_string());
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
        ui.heading("Server ".to_owned() + &*node.node_id.to_string());
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



