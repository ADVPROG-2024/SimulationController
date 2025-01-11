mod scheme;
mod leftSideBar;

use std::collections::{HashMap, HashSet};
use eframe::egui;
use rand::Rng;

fn main() {
    let mut nodi = Vec::new();
    let s1 = Node::new(1, NodeType::SERVER, &mut nodi);
    let c1 = Node::new(2, NodeType::CLIENT, &mut nodi);
    let d1 = Node::new(3, NodeType::DRONE, &mut nodi);
    let d2 = Node::new(4, NodeType::DRONE, &mut nodi);
    let d3 = Node::new(5, NodeType::DRONE, &mut nodi);
    let d4 = Node::new(6, NodeType::DRONE, &mut nodi);
    let d5 = Node::new(7, NodeType::DRONE, &mut nodi);
    let d6 = Node::new(8, NodeType::DRONE, &mut nodi);
    let d7 = Node::new(9, NodeType::DRONE, &mut nodi);
    let d8 = Node::new(10, NodeType::DRONE, &mut nodi);
    let d9 = Node::new(11, NodeType::DRONE, &mut nodi);
    let d10 = Node::new(12, NodeType::DRONE, &mut nodi);
    let s2 = Node::new(13, NodeType::SERVER, &mut nodi);


    let mut neighbours: HashSet<(u8, u8)> = HashSet::new();
    neighbours.insert((s1.node_id, c1.node_id));
    neighbours.insert((s1.node_id, d1.node_id));
    neighbours.insert((c1.node_id, d1.node_id));
    neighbours.insert((d1.node_id, d2.node_id));
    neighbours.insert((d1.node_id, s2.node_id));

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Simulation Controller",
        native_options,
        Box::new(|cc| Ok(Box::new(SimulationController::new(cc, nodi, neighbours)))),
    );
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum NodeType {
    SERVER,
    CLIENT,
    DRONE,
}

#[derive(Clone)]
struct Node {
    node_id: u8,
    node_type: NodeType,
    xy: (f32, f32),
}

impl Node {
    fn new(node_id: u8, node_type: NodeType, nodi: &mut Vec<Node>) -> Self {
        let node = Self {
            node_id,
            node_type,
            xy: Self::set_coordinates(nodi),
        };
        nodi.push(node.clone());
        node
    }

    fn set_coordinates(nodi: &Vec<Node>) -> (f32,f32){
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
struct SimulationController {
    nodi: Vec<Node>,
    neighbours: HashSet<(u8,u8)>,
    left_panel: bool,
}

struct SimulationControllerFromNetwork{

}

impl SimulationController {

    fn create<T>(node_vec: HashSet<T>, sim_command_channels: HashMap<NodeId, Sender<DroneCommands>>) -> Self{

    }
    fn new(cc: &eframe::CreationContext<'_>, nodi: Vec<Node>, neighbours: HashSet<(u8, u8)>) -> Self {
        Self {
            nodi,
            neighbours,
            left_panel: false,
        }
    }
}

impl eframe::App for SimulationController {
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