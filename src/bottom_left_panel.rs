use std::fmt::Debug;
use dronegowski_utils::hosts::ClientEvent;
use eframe::egui;
use wg_2024::controller::DroneEvent;
use wg_2024::packet::PacketType;
use crate::DronegowskiSimulationController;
use crate::sc_utils::Event;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        for elem in &self.panel.bottom_left_panel.event{
            match elem {
                Event::DroneEvent(drone_event) => {
                    match drone_event {
                        DroneEvent::PacketSent(info) => {
                            match info.pack_type {
                                PacketType::MsgFragment(_) => {}
                                PacketType::Ack(_) => {}
                                PacketType::Nack(_) => {}
                                PacketType::FloodRequest(_) => {}
                                PacketType::FloodResponse(_) => {}
                            }
                            ui.label("Drone event");
                        }
                        DroneEvent::PacketDropped(_) => {}
                        DroneEvent::ControllerShortcut(_) => {}
                    }

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
