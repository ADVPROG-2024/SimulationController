use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController<'_> {
    pub fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui.heading("SIMULATION CONTROLLER".to_string());

        if ui.button("Spawn").clicked() && self.panel.bottom_panel.spawn_pdr != "" {
            let pdr = self.panel.bottom_panel.spawn_pdr.clone().parse::<f32>().unwrap();
            if pdr <= 1. && pdr >= 0. {
                self.spawn(pdr);
            }
        }
        ui.text_edit_singleline(&mut self.panel.bottom_panel.spawn_pdr);


        if ui.button("Send").clicked(){
            self.send_packet_test();
        }


    }

    pub fn bottom_panel_drone(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());
        ui.horizontal(|ui| {
            if ui.button("Add Sender").clicked() {
                self.panel.bottom_panel.add_sender = true;
            }
            if ui.button("Remove Sender").clicked() {
                self.panel.bottom_panel.remove_sender = true;
            }
            ui.vertical(|ui|{
                if ui.button("Set PDR").clicked() && self.panel.bottom_panel.change_pdr != "" {
                    let pdr = self.panel.bottom_panel.change_pdr.clone().parse::<f32>().unwrap();
                    if pdr <= 1. && pdr >= 0. {
                        self.set_pdr(pdr);
                    }
                }
                ui.text_edit_singleline(&mut self.panel.bottom_panel.change_pdr);
            });
            if ui.button("Crash").clicked() {
                self.panel.bottom_panel.crash = true;
            }
        });
    }

    pub fn bottom_panel_client(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());
        if ui.button("Add Sender").clicked(){
            self.panel.bottom_panel.add_sender = true;
        }
        if ui.button("Remove Sender").clicked(){
            self.panel.bottom_panel.remove_sender = true;
        }
    }

    pub fn bottom_panel_server(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());
        if ui.button("Add Sender").clicked(){
            self.panel.bottom_panel.add_sender = true;
        }
        if ui.button("Remove Sender").clicked(){
            self.panel.bottom_panel.remove_sender = true;
        }
    }
}



