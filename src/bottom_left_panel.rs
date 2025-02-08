use std::fmt::Debug;
use eframe::egui;
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        for elem in &self.panel.bottom_left_panel.drone_event{
            ui.label(format!("{:?}", elem));
        }

        for elem in &self.panel.bottom_left_panel.client_event{
            ui.label(format!("{:?}", elem));

        }

        for elem in &self.panel.bottom_left_panel.server_event{
            //ui.label(format!("{:?}", elem));

        }
    }
}
