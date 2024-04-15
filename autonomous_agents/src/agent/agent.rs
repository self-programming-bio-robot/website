use std::cell::RefMut;
use std::collections::HashMap;
use crate::map::map::Map;

pub struct SimpleAgent {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub energy: usize,
    pub vision_range: usize,
    pub knowledge: String,
    pub memory: String,
}

pub trait Agent {
    fn update(&self, map: &mut Map, tick: usize, agents: RefMut<&HashMap<String, Box<dyn Agent>>>);
    fn name(&self) -> String;
    
    fn move_to(&mut self, x: usize, y: usize, map: &Map) -> bool {
        if let Some(cell) = map.get_cell(x, y) {
            if !cell.passable {
                return false;
            }
            self.translate(x, y);
            
            true
        } else {
            false
        }
    }
    
    fn translate(&mut self, x: usize, y: usize);
}

impl Agent for SimpleAgent {
    fn update(&self, map: &mut Map, tick: usize, agents: RefMut<&HashMap<String, Box<dyn Agent>>>) {
        println!("Agent {} is updating", self.name);
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
    
    fn translate(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}