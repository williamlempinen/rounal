use std::sync::{Arc, RwLock};

use crate::{journal, util};

use anyhow::Result;

pub enum View {
    Jounrnalctl,
    Systemctl,
}

struct App {
    pub quit: bool,
    pub logs: Arc<RwLock<Vec<String>>>,
    pub services: Arc<RwLock<Vec<String>>>,
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
