use std::collections::HashMap;
use std::fs;
use wg_2024;
use crossbeam_channel::{unbounded, Receiver, Sender};
use dronegowski::Dronegowski;
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
    pub sim_command_channels: HashMap<NodeId, Sender<DroneCommand>>,
    pub sim_event_recv: Receiver<DroneEvent>
}

impl DronegowskiSimulationController {
    pub fn new(config: Config, sim_command_channels: HashMap<NodeId, Sender<DroneCommand>>, sim_event_recv: Receiver<DroneEvent>){
        let mut nodi = Vec::new();
        Self::parse_file(config, &mut nodi);

        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi, sim_command_channels, sim_event_recv)))),
        ).expect("Error to run the Simulation Controller");
    }

    fn create(cc: &eframe::CreationContext<'_>, nodi: Vec<SimulationControllerNode>, sim_command_channels: HashMap<NodeId, Sender<DroneCommand>>, sim_event_recv: Receiver<DroneEvent>) -> Self {
        Self {
            nodi,
            sim_command_channels,
            sim_event_recv
        }
    }

    fn parse_file(config: Config,  nodi: &mut Vec<SimulationControllerNode>){
        for drone in config.drone.clone(){
            let mut neighbours = Vec::new();
            for neighbour in drone.connected_node_ids{
                neighbours.push(neighbour);
            }
            SimulationControllerNode::new(SimulationControllerNodeType::DRONE, drone.id, neighbours, nodi);
        }

        for client in config.client{
            let mut neighbours = Vec::new();
            for neighbour in client.connected_drone_ids{
                neighbours.push(neighbour);
            }
            SimulationControllerNode::new(SimulationControllerNodeType::CLIENT, client.id, neighbours, nodi);
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