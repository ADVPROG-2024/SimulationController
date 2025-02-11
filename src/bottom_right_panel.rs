use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use dronegowski_utils::network::{Event, SimulationControllerNodeType};
use eframe::egui;
use eframe::egui::{Color32, Direction, Layout, RichText};
use wg_2024::controller::DroneEvent;
use wg_2024::packet::{NackType, PacketType};
use crate::DronegowskiSimulationController;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_right_panel(&mut self, ui: &mut egui::Ui){
        ui.add_space(20.);
        if let Some(node) = &self.panel.central_panel.selected_node {
            match node.node_type{
                SimulationControllerNodeType::SERVER { .. } => {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                            ui.heading(RichText::new(format!("SERVER {}", node.node_id)).size(25.0).color(Color32::RED));
                        });
                    });
                    ui.add_space(30.);
                }
                SimulationControllerNodeType::CLIENT { .. } => {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                            ui.heading(RichText::new(format!("CLIENT {}", node.node_id)).size(25.0).color(Color32::GREEN));
                        });
                    });
                    ui.add_space(30.);
                }
                SimulationControllerNodeType::DRONE { .. } => {
                    ui.horizontal(|ui| {
                        ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                            ui.heading(RichText::new(format!("DRONE {}", node.node_id)).size(25.0).color(Color32::BLUE));
                        });
                    });
                    ui.add_space(30.);
                }
            }

            for elem in &self.panel.bottom_left_panel.event{
                match elem {
                    Event::DroneEvent(drone_event) => {
                        match drone_event {
                            DroneEvent::PacketSent (packet) => {
                                match packet.clone().pack_type {
                                    PacketType::MsgFragment(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_drone_notify(format!("Drone {} sent a fragment to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Ack(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_drone_notify(format!("Drone {} sent an ACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Nack(nack) => {
                                        match nack.nack_type {

                                            NackType::ErrorInRouting(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_drone_notify(format!("Drone {} sent an ErrorInRouting NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::DestinationIsDrone => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_drone_notify(format!("Drone {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::Dropped => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_drone_notify(format!("Drone {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::UnexpectedRecipient(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_drone_notify(format!("Drone {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            DroneEvent::PacketDropped(packet) => {
                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                if node_id == node.node_id {
                                    self.print_drone_notify(format!("Drone {} dropped a fragment", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                }
                            }
                            DroneEvent::ControllerShortcut(packet) => {
                                match packet.pack_type {
                                    PacketType::Ack(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_drone_notify(format!("Drone {} sent directly to SC an ACK", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Nack(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_drone_notify(format!("Drone {} sent directly to SC a NACK", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::FloodResponse(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_drone_notify(format!("Drone {} sent directly to SC a FloodResponse", packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    _ => {}
                                }
                                let node_id = packet.routing_header.hops[packet.routing_header.hops.len() - 1];
                                let node_index = self.nodi.iter().position(|node| node.node_id == node_id);
                                if let Some(node_idx) = node_index{
                                    let node = self.nodi[node_idx].clone();
                                    match node.node_type{
                                        SimulationControllerNodeType::SERVER { .. } => {
                                            if let Some(channel) = self.sc_server_channels.get(&node.node_id){
                                                channel.send(ServerCommand::ControllerShortcut(packet.clone())).expect("Impossible send ControllerShortcut to Client");
                                            }
                                        }
                                        SimulationControllerNodeType::CLIENT { .. } => {
                                            if let Some(channel) = self.sc_client_channels.get(&node.node_id) {
                                                channel.send(ClientCommand::ControllerShortcut(packet.clone())).expect("Impossible send ControllerShortcut to Client");
                                            }
                                        }
                                        _ =>{}
                                    }
                                }
                            }
                        }
                    }

                    Event::ClientEvent (client_event) => {
                        match client_event {
                            ClientEvent::PacketSent (packet) => {

                                match packet.clone().pack_type {
                                    PacketType::MsgFragment(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_client_notify(format!("Client {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Ack(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_client_notify(format!("Client {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Nack(nack) => {
                                        match nack.nack_type {
                                            NackType::ErrorInRouting(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_client_notify(format!("Client {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::DestinationIsDrone => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_client_notify(format!("Client {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::Dropped => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_client_notify(format!("Client {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::UnexpectedRecipient(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_client_notify(format!("Client {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }

                    Event::ServerEvent (server_event) => {
                        match server_event {
                            ServerEvent::PacketSent (packet) => {

                                match packet.clone().pack_type {
                                    PacketType::MsgFragment(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_server_notify(format!("Server {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Ack(_) => {
                                        let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                        if node_id == node.node_id {
                                            self.print_server_notify(format!("Server {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                        }
                                    }
                                    PacketType::Nack(nack) => {
                                        match nack.nack_type {
                                            NackType::ErrorInRouting(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_server_notify(format!("Server {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::DestinationIsDrone => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_server_notify(format!("Server {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::Dropped => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_server_notify(format!("Server {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                            NackType::UnexpectedRecipient(_) => {
                                                let node_id = packet.routing_header.hops[packet.routing_header.hop_index-1];
                                                if node_id == node.node_id {
                                                    self.print_server_notify(format!("Server {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index - 1], packet.routing_header.hops[packet.routing_header.hop_index]), ui);
                                                }
                                            }
                                        }
                                    }

                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
}