use std::collections::HashMap;
use std::time::Instant;
use dronegowski_utils::functions::ValidationError;
use dronegowski_utils::network::{SimulationControllerNode, Event};
use wg_2024::network::NodeId;

pub struct Panel{
    pub central_panel: CentralPanel,
    pub upper_left_panel: UpperLeftPanel,
    pub bottom_left_panel: BottomLeftPanel,
}

pub struct CentralPanel {
    pub selected_node: Option<SimulationControllerNode>,
    pub active_popups: HashMap<NodeId, SimulationControllerNode>,
    pub active_error: Result<(), ValidationError>,
    pub popup_timer: Option<Instant>,
}


pub struct UpperLeftPanel {
    pub add_sender: bool,
    pub remove_sender: bool,
    pub crash: bool,
    pub change_pdr: String,
    pub spawn_pdr: String,
}

pub struct BottomLeftPanel {
    pub event: Vec<Event>,
    pub index: usize,
}


impl Panel{
    pub fn default() -> Self {
        Self{
            central_panel: CentralPanel::new(),
            upper_left_panel: UpperLeftPanel::new(),
            bottom_left_panel: BottomLeftPanel::new(),
        }
    }

    pub fn reset(&mut self){
        self.upper_left_panel.reset();
    }
}

impl UpperLeftPanel{
    fn new() -> Self{
        Self{
            add_sender: false,
            remove_sender: false,
            crash: false,
            change_pdr: "".to_string(),
            spawn_pdr: "".to_string(),
        }
    }

    fn reset(&mut self){
        self.add_sender = false;
        self.remove_sender = false;
        self.crash = false;
        self.change_pdr = "".to_string();
        self.spawn_pdr = "".to_string();
    }
}

impl BottomLeftPanel{
    fn new() -> Self{
        Self{
            event: Vec::new(),
            index: 0,
        }
    }
}

impl CentralPanel{
    fn new() -> Self{
        Self{
            selected_node: None,
            active_popups: HashMap::new(),
            active_error: Ok(()),
            popup_timer: None,

        }
    }
}