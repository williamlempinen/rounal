use std::sync::{Arc, RwLock};

use crate::{journal, system::Service, util};

use anyhow::Result;

pub enum View {
    Jounrnalctl,
    Systemctl,
}

struct App {
    pub quit: bool,
    pub logs: Arc<RwLock<Vec<String>>>,
    pub services: Arc<RwLock<Vec<Service>>>,
    pub current_view: View,
    pub modal_visible: bool,
}

impl App {
    fn new() -> Self {
        todo!("TODO");
    }
}

pub fn start() -> Result<()> {
    Ok(())
}

fn run() -> Result<()> {
    Ok(())
}
