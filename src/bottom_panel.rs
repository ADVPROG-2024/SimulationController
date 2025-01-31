use std::cell::RefCell;
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Color32, Pos2, Stroke};
use wg_2024::network::NodeId;
use crate::{DronegowskiSimulationController};

impl DronegowskiSimulationController {
    pub fn bottom_panel(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.add_sized([150.0, 50.0], egui::Button::new("Ciao")).clicked() {}
        });

        ui.heading("Esempio di UI");

        ui.horizontal(|ui| {
            ui.label("Nome:");
            ui.text_edit_singleline(&mut "Inserisci un nome".to_string());
        });

        egui::ComboBox::from_label("Scegli un'opzione")
            .selected_text(&"Si".to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut "Si".to_string(), "Opzione 1".to_string(), "Opzione 1");
                ui.selectable_value(&mut "No".to_string(), "Opzione 2".to_string(), "Opzione 2");
            });

        ui.checkbox(&mut false, "Attiva funzione");

        ui.add(egui::Slider::new(&mut 0., 0.0..=100.0).text("Intensità"));

        if ui.button("Conferma").clicked() {}

    }

    pub fn bottom_panel_drone(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());

        /*egui::ComboBox::new(egui::Id::new("combo_box_select_comand"), "")
            .selected_text(&self.panel.bottom_panel.command_select)
            .show_ui(ui, |ui| {
                for option in &["Add Sender", "Remove Sender", "Crash", "Set PDR"] {
                    if ui.selectable_label(self.panel.bottom_panel.command_select == *option, *option).clicked() {
                        self.panel.bottom_panel.command_select = option.to_string();
                        println!("{}", self.panel.bottom_panel.command_select);
                    }
                }
            });

        if &self.panel.bottom_panel.command_select == "Add Sender" {

            self.panel.bottom_panel.active_add_sender = true;
            let mut filtered_nodes: Vec<_> = self.nodi.iter()
                .filter(|node| {
                    if let Some(selected) = &self.panel.central_panel.selected_node {
                        // Escludi il nodo selezionato e i suoi vicini
                        node.node_id != selected.node_id && !selected.neighbours.contains(&node.node_id)
                    } else {
                        true
                    }
                })
                .collect();
            egui::ComboBox::new(egui::Id::new("combo_box_add_sender"), "")
                .selected_text(&self.panel.bottom_panel.selected_node_operation)
                .show_ui(ui, |ui| {
                    for elem in filtered_nodes{
                        if ui.selectable_value(&mut self.panel.bottom_panel.selected_node_operation, elem.node_id.to_string(), elem.node_id.to_string()).clicked() {
                            self.panel.bottom_panel.selected_node_operation = elem.node_id.to_string();
                        }
                    }


                });
            }*/
            if ui.button("Add Sender").clicked(){
                self.panel.bottom_panel.active_add_sender = true;
            }
            if ui.button("Remove Sender").clicked(){
                //self.add_sender();
            }
            if ui.button("Set PDR").clicked(){
                //self.add_sender();
            }
            if ui.button("Crash").clicked(){
                //self.add_sender();
            }

    }

    pub fn bottom_panel_client(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());
    }

    pub fn bottom_panel_server(&mut self, ui: &mut egui::Ui, node: SimulationControllerNode) {
        ui.heading(node.node_id.to_string());
    }
}



