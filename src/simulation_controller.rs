use std::collections::HashMap;
use wg_2024;
use crossbeam_channel::{Receiver, Sender};
use dronegowski_utils::hosts::{ClientCommand, ClientEvent, ServerCommand, ServerEvent};
use eframe::egui;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use dronegowski_utils::network::{SimulationControllerNode};

pub struct DronegowskiSimulationController {
    pub nodi: Vec<SimulationControllerNode>,
    pub sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
    pub sc_server_channels: HashMap<NodeId, Sender<ServerCommand>>,
    pub sc_drone_event_recv: Receiver<DroneEvent>,
    pub sc_client_event_recv: Receiver<ClientEvent>,
    pub sc_server_event_recv: Receiver<ServerEvent>,
    pub active_popups: HashMap<NodeId, SimulationControllerNode>, // Popup attivi
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
            sc_server_event_recv,
            active_popups: HashMap::new(), // Inizializzazione vuota
        }
    }

    fn open_client_popup(&mut self, node: &SimulationControllerNode) {
        // Aggiungi un popup per il nodo specifico
        self.active_popups.insert(node.node_id, node.clone());
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
        let mut popups_to_remove = vec![];
        for (node_id, node) in &self.active_popups {
            egui::Window::new(format!("Client: {}", node_id))
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label(format!("ID: {}", node.node_id));
                    ui.label(format!("Type: {:?}", node.node_type));
                    // Aggiungi ulteriori informazioni qui
                    if ui.button("Chiudi").clicked() {
                        popups_to_remove.push(*node_id);
                    }
                });
        }

        // Rimuovi i popup chiusi
        for node_id in popups_to_remove {
            self.active_popups.remove(&node_id);
        }
    }
}