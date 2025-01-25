use std::collections::HashSet;
use std::fs;
use wg_2024;
use crossbeam_channel::{unbounded, Receiver, Sender};
use dronegowski::Dronegowski;
use eframe::{egui, EventLoopBuilderHook};
use rand::Rng;
use wg_2024::controller::{DroneCommand, DroneEvent};
use wg_2024::network::NodeId;
use wg_2024::config::Config;
//use winit::platform::wayland::EventLoopBuilderExtWayland;

#[test]
fn test(){
    let config = parse_config("config.toml");
    DronegowskiSimulationController::new(config);
}

pub fn parse_config(file: &str) -> Config {
    let file_str = fs::read_to_string(file).expect("error reading config file");
    println!("Parsing configuration file...");
    toml::from_str(&file_str).expect("Error occurred during config file parsing")
}
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    SERVER,
    CLIENT,
    DRONE,
}

#[derive(Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub node_id: NodeId,
    pub neighbours: HashSet<NodeId>,
    pub xy: (f32, f32),
}

impl Node {
    fn new(node_type: NodeType, node_id: NodeId, neighbours: HashSet<NodeId>, nodi: &mut Vec<Node>) -> Self {
        let node = Self {
            node_type,
            node_id,
            neighbours,
            xy: Self::set_coordinates(nodi),
        };
        nodi.push(node.clone());
        node
    }

    fn set_coordinates(nodi: &mut Vec<Node>) -> (f32, f32){
        let mut x;
        let mut y;
        loop{
            x = rand::thread_rng().gen_range(50. ..550.);
            y = rand::thread_rng().gen_range(50. ..550.);
            if !nodi.iter().any(|node| {
                let dist = ((node.xy.0 - x).powi(2) + (node.xy.1 - y).powi(2)).sqrt();
                dist < 100.}) {
                break;
            }
        }
        (x, y)
    }
}

#[derive(Default)]
pub struct DronegowskiSimulationController {
    //sim_controller_event_recv: Receiver<DroneEvent>,
    //sim_controller_command_send: HashMap<NodeId, Sender<DroneCommand>>,
    pub nodi: HashSet<Node>,
    pub left_panel: bool,
}

impl DronegowskiSimulationController {
    //nodes_channels: HashMap<NodeId, Sender<DroneCommand>>, sim_controller_event_recv: Receiver<DroneEvent>, sim_controller_event_send: Sender<DroneCommand>,
    pub fn new(config: Config){
        let mut nodi = HashSet::new();
        Self::parse_file(config, &mut nodi);

        /*let event_loop_builder: Option<EventLoopBuilderHook> = Some(Box::new(|event_loop_builder| {
            event_loop_builder.with_any_thread(true);
        }));

        let native_options = eframe::NativeOptions {
            event_loop_builder,
            ..Default::default()
        };*/
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Simulation Controller",
            native_options,
            Box::new(|cc| Ok(Box::new(DronegowskiSimulationController::create(cc, nodi)))),
        );
    }

    fn create(cc: &eframe::CreationContext<'_>, nodi: HashSet<Node>) -> Self {
        Self {
            nodi,
            left_panel: false,
        }
    }

    fn parse_file(config: Config,  nodi: &mut Vec<Node>){
        for drone in config.drone{
            let mut neighbours = HashSet::new();
            for neighbour in drone.connected_node_ids{
                neighbours.insert(neighbour);
            }
            Node::new(NodeType::DRONE, drone.id, neighbours, nodi);
        }

        for client in config.client{
            let mut neighbours = HashSet::new();
            for neighbour in client.connected_drone_ids{
                neighbours.insert(neighbour);
            }
            Node::new(NodeType::CLIENT, client.id, neighbours, nodi);
        }

        for server in config.server{
            let mut neighbours = HashSet::new();
            for neighbour in server.connected_drone_ids{
                neighbours.insert(neighbour);
            }
            Node::new(NodeType::SERVER, server.id, neighbours, nodi);
        }
    }
}

impl eframe::App for DronegowskiSimulationController {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        if self.left_panel{
            egui::SidePanel::left("side_panel").resizable(false).show(ctx, |ui| {
                self.left_side_panel(ui);
            });
        }

        egui::CentralPanel::default().frame(egui::Frame::none()).show(ctx, |ui| {
            self.central_panel(ui);
        });
    }
}