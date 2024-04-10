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
}