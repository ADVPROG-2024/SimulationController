use std::collections::HashMap;
use std::fs;
use wg_2024;
use crossbeam_channel::{unbounded, Receiver, Sender};
use dronegowski::Dronegowski;
use dronegowski_utils::hosts::{ClientCommand, ClientEvent};
use eframe::egui;
use rand::Rng;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use wg_2024::config::Config;
use dronegowski_utils::network::{SimulationControllerNode, SimulationControllerNodeType};

pub fn parse_config(file: &str) -> Config {
    let file_str = fs::read_to_string(file).expect("error reading config file");
    println!("Parsing configuration file...");
    toml::from_str(&file_str).expect("Error occurred during config file parsing")
}

pub struct DronegowskiSimulationController {
    pub nodi: Vec<SimulationControllerNode>,
    pub sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
    pub sc_drone_event_recv: Receiver<DroneEvent>,
    pub sc_client_event_recv: Receiver<ClientEvent>
}

impl DronegowskiSimulationController {
    pub fn new(config: Config,
               sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
               sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
               sc_drone_event_recv: Receiver<DroneEvent>,
               sc_client_event_recv: Receiver<ClientEvent>){
        let mut nodi = Vec::new();
        Self::parse_file(config, &mut nodi, sc_drone_channels, sc_client_channels);

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi, sc_drone_channels, sc_client_channels, sc_drone_event_recv, sc_client_event_recv)))),
        ).expect("Error to run the Simulation Controller");
    }

    fn create(cc: &eframe::CreationContext<'_>,
              nodi: Vec<SimulationControllerNode>,
              sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
              sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,
              sc_drone_event_recv: Receiver<DroneEvent>,
              sc_client_event_recv: Receiver<ClientEvent>) -> Self {
        Self {
            nodi,
            sc_drone_channels,
            sc_client_channels,
            sc_drone_event_recv,
            sc_client_event_recv,
        }
    }

    fn parse_file(config: Config,
                  nodi: &mut Vec<SimulationControllerNode>,
                  sc_drone_channels: HashMap<NodeId, Sender<DroneCommand>>,
                  sc_client_channels: HashMap<NodeId, Sender<ClientCommand>>,){

        for drone in config.drone.clone(){
            let mut neighbours = Vec::new();
            let Some(sc_drone_command) = sc_drone_channels.get(&drone.id);

            for neighbour in drone.connected_node_ids{
                neighbours.push(neighbour);
            }
            SimulationControllerNode::new(SimulationControllerNodeType::DRONE{ drone_channel: *sc_drone_command.clone(), pdr: drone.pdr }, drone.id, neighbours, nodi);
        }

        for client in config.client{
            let mut neighbours = Vec::new();
            let Some(sc_client_command) = sc_client_channels.get(&client.id);

            for neighbour in client.connected_drone_ids{
                neighbours.push(neighbour);
            }

            SimulationControllerNode::new(SimulationControllerNodeType::CLIENT{ client_channel: *sc_client_command.clone()}, client.id, neighbours, nodi);
        }

        for server in config.server{
            let mut neighbours = Vec::new();
            for neighbour in server.connected_drone_ids{
                neighbours.push(neighbour);
            }
            SimulationControllerNode::new(SimulationControllerNodeType::SERVER, server.id, neighbours, nodi);
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