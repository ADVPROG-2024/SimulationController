use std::fmt::Debug;
use dronegowski_utils::hosts::ClientEvent;
use eframe::egui;
use wg_2024::controller::DroneEvent;
use crate::DronegowskiSimulationController;
use crate::sc_utils::Event;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        for elem in &self.panel.bottom_left_panel.event{
            match elem {
                Event::DroneEvent{..} => {
                    ui.label("Drone event");
                }
                Event::ClientEvent {..} => {
                    ui.label("Client event");

                }
                Event::ServerEvent {..} => {
                    ui.label("Server event");

                }
            }
        }
    }
}
