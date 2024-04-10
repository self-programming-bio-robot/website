use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;

use crate::agent::agent::Agent;
use crate::map::map::Map;

pub struct World {
    pub agents: HashMap<String, Box<dyn Agent>>,
    pub map: Map,
    pub ticks: usize,
}

impl World {
    pub fn new(agents: Vec<Box<dyn Agent>>, map_file_name: &str) -> Result<World, Box<dyn Error>> {
        let map = Map::load_from_file(map_file_name)?;
        let agents = agents.into_iter()
            .map(|agent| (agent.name(), agent)).collect();
        Ok(World {
            agents,
            map,
            ticks: 0,
        })
    }
    
    pub fn tick(&mut self) {
        let mut agents_ref = RefCell::new(&self.agents);
        for agent in self.agents.values().into_iter() {
            agent.update(&mut self.map, self.ticks, agents_ref.borrow_mut());
        }
        self.ticks += 1;
    }
}