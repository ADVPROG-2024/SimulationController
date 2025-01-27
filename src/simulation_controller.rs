use std::collections::HashMap;
use wg_2024;
use crossbeam_channel::{Receiver, Sender};
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use eframe::egui;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};

pub struct DronegowskiSimulationController {
    pub nodi: Vec<SimulationControllerNode>,
    pub sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
    pub sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
    pub sc_drone_event_recv: Receiver<DroneEvent>,
    pub sc_client_event_recv: Receiver<ClientEvent>,
    pub sc_server_event_recv: Receiver<ServerEvent>
}

impl DronegowskiSimulationController {
    pub fn new(nodi: Vec<SimulationControllerNode>,
               sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
               sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
               sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
               sc_drone_event_recv: Receiver<DroneEvent>,
               sc_client_event_recv: Receiver<ClientEvent>,
               sc_server_event_recv: Receiver<ServerEvent>
    ){

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi, sc_drone_channels, sc_client_channels, sc_server_channels, sc_drone_event_recv, sc_client_event_recv, sc_server_event_recv)))),
        ).expect("Error to run the Simulation Controller");
    }

    fn create(cc: &eframe::CreationContext<'_>,
              nodi: Vec<SimulationControllerNode>,
              sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
              sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
              sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
              sc_drone_event_recv: Receiver<DroneEvent>,
              sc_client_event_recv: Receiver<ClientEvent>,
              sc_server_event_recv: Receiver<ServerEvent>
    ) -> Self {
        Self {
            nodi,
            sc_drone_channels,
            sc_client_channels,
            sc_server_channels,
            sc_drone_event_recv,
            sc_client_event_recv,
            sc_server_event_recv
        }
    }
}

impl eframe::App for DronegowskiSimulationController {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
            self.left_side_panel(ui);
        });

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            self.central_panel(ui);
        });
    }
}