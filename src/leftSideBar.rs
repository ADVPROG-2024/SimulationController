use eframe::egui;
use crate::SimulationController;

impl SimulationController{
    pub fn left_side_panel(&mut self, ui: &mut egui::Ui){
        for node in &self.nodi {
            ui.label(format!(
                "Nodo {}: {:?} ({}, {})",
                node.node_id, node.node_type, node.xy.0, node.xy.1
            ));
        }
    }
}