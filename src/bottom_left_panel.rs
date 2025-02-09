use std::fmt::Debug;
use dronegowski_utils::hosts::{ClientEvent, ServerEvent};
use eframe::egui;
use wg_2024::controller::DroneEvent;
use wg_2024::packet::{NackType, PacketType};
use crate::DronegowskiSimulationController;
use crate::sc_utils::Event;

impl DronegowskiSimulationController<'_> {
    pub fn bottom_left_panel(&mut self, ui: &mut egui::Ui){
        ui.heading("NOTIFICHE".to_string());

        for elem in &self.panel.bottom_left_panel.event{

            match elem {
                Event::DroneEvent(drone_event) => {
                    match drone_event {
                        DroneEvent::PacketSent (packet) => {
                            match packet.clone().pack_type {
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Drone {} sent a fragment to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Drone {} sent an ACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Drone {} sent an ErrorInRouting NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Drone {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Drone {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Drone {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                    }
                                }
                                // PacketType::FloodRequest(_) => {
                                //     ui.label(format!("Drone {} sent a FloodRequest", node_id_receiver));
                                // }
                                // PacketType::FloodResponse(_) => {
                                //     ui.label(format!("Drone {} sent a FloodResponse", node_id_receiver));
                                // }
                                _ => {}
                            }
                        }
                        DroneEvent::PacketDropped(packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            ui.label(format!("Drone {} dropped a Packet: {:?}", node_id_receiver, packet.clone().pack_type));
                        }
                        DroneEvent::ControllerShortcut(packet) => {
                            let node_id_receiver = packet.routing_header.hops[packet.routing_header.hop_index];
                            match packet.pack_type {
                                PacketType::Ack(_) => {
                                    ui.label(format!("Drone {} sent directly to Simulation Controller an ACK", node_id_receiver));
                                }
                                PacketType::Nack(_) => {
                                    ui.label(format!("Drone {} sent directly to Simulation Controller a NACK", node_id_receiver));
                                }
                                PacketType::FloodResponse(_) => {
                                    ui.label(format!("Drone {} sent directly to Simulation Controller a FloodResponse", node_id_receiver));
                                }
                                _ => {}
                            }
                        }
                    }
                }

                Event::ClientEvent (client_event) => {
                    match client_event {
                        ClientEvent::PacketSent (packet) => {
                            match packet.clone().pack_type {
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Client {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Client {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Client {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Client {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Client {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Client {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                    }
                                }
                                // PacketType::FloodRequest(_) => {
                                //     ui.label(format!("Client {} received a FloodRequest from Node {}", node_id_receiver, node_id_sender));
                                // }
                                // PacketType::FloodResponse(_) => {
                                //     ui.label(format!("Client {} received a FloodResponse from Node {}", node_id_receiver, node_id_sender));
                                // }
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
                                PacketType::MsgFragment(fragment) => {
                                    ui.label(format!("Server {} sent a fragment to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Ack(_) => {
                                    ui.label(format!("Server {} sent an ACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                }
                                PacketType::Nack(nack) => {
                                    match nack.nack_type {
                                        NackType::ErrorInRouting(_) => {
                                            ui.label(format!("Server {} sent an ErrorInRouting NACK to Drone {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::DestinationIsDrone => {
                                            ui.label(format!("Server {} sent a DestinationIsDrone NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::Dropped => {
                                            ui.label(format!("Server {} sent a Dropped NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                        NackType::UnexpectedRecipient(_) => {
                                            ui.label(format!("Server {} sent an UnexpectedRecipient NACK to Node {}", packet.routing_header.hops[packet.routing_header.hop_index-1], packet.routing_header.hops[packet.routing_header.hop_index]));
                                        }
                                    }
                                }
                                // PacketType::FloodRequest(_) => {
                                //     ui.label(format!("Client {} received a FloodRequest from Node {}", node_id_receiver, node_id_sender));
                                // }
                                // PacketType::FloodResponse(_) => {
                                //     ui.label(format!("Client {} received a FloodResponse from Node {}", node_id_receiver, node_id_sender));
                                // }
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
