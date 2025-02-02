use dronegowski_utils::network::SimulationControllerNode;
use eframe::epaint::Color32;
use wg_2024::network::NodeId;

pub struct Panel{
    pub central_panel: CentralPanel,
    pub bottom_panel: BottomPanel,
    pub left_panel: LeftPanel
}


pub struct CentralPanel {
    pub selected_node: Option<SimulationControllerNode>,
}

pub struct BottomPanel {
    pub add_sender: bool,
    pub remove_sender: bool,
    pub crash: bool,
    pub change_pdr: String,
    pub spawn_pdr: String,
}

pub struct LeftPanel {
    pub active_left_button: LeftButton,
    pub left_button: String
}

#[derive(PartialEq)]
pub enum LeftButton{
    Network,
    Notifiche,
    Lista
}
impl Panel{
    pub fn default() -> Self {
        Self{
            central_panel: CentralPanel::new(),
            bottom_panel: BottomPanel::new(),
            left_panel: LeftPanel::new()
        }
    }

    pub fn reset(&mut self){
        self.bottom_panel.reset();
    }
}

impl BottomPanel{
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

impl LeftPanel{
    fn new() -> Self{
        Self{
            active_left_button: LeftButton::Network,
            left_button: "Visualizzazione network".to_string()
        }
    }
}

impl CentralPanel{
    fn new() -> Self{
        Self{
            selected_node: None,
        }
    }
}