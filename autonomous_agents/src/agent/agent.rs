use std::cell::{OnceCell, RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use langchain_rust::agent::{ConversationalAgentBuilder, OpenAiToolAgentBuilder};
use langchain_rust::language_models::llm::LLM;
use langchain_rust::llm::openai::{OpenAI, OpenAIModel};
use langchain_rust::tools::CommandExecutor;
use crate::agent::tools::move_action::Move;
use crate::map::cell::Cell;
use crate::map::map::Map;
use crate::map::object::Object;

pub struct SimpleAgent {
    pub name: String,
    pub x: usize,
    pub y: usize,
    pub energy: usize,
    pub vision_range: usize,
    pub knowledge: String,
    pub memory: String,
    pub self_agent: Arc<Mutex<Option<SimpleAgent>>>,
}

pub trait Agent: Send + Sync {
    fn update(&self, map: Arc<Mutex<Map>>, tick: usize, agents: RefMut<&HashMap<String, Box<dyn Agent>>>);
    fn name(&self) -> String;

    fn move_to(&mut self, x: usize, y: usize, map: &Map) -> bool {
        if let Some(cell) = map.get_cell(x, y) {
            if let Ok(cell) = cell.lock() {
                if !cell.passable {
                    return false;
                }
                self.translate(x, y);

                true
            } else {
                false
            }
        } else {
            false
        }
    }
    
    fn move_up(&mut self, map: &Map) -> bool;
    
    fn move_down(&mut self, map: &Map) -> bool;
    
    fn move_left(&mut self, map: &Map) -> bool;
    
    fn move_right(&mut self, map: &Map) -> bool;
    
    fn translate(&mut self, x: usize, y: usize);
}

impl Agent for SimpleAgent {
    fn update(&self, map: Arc<Mutex<Map>>, tick: usize, agents: RefMut<&HashMap<String, Box<dyn Agent>>>) {
        let llm = OpenAI::default().with_model(OpenAIModel::Gpt35);
        let move_action = Arc::new(
            Move::new(self.self_agent.clone(), map.clone())
        );
        
        let agent = ConversationalAgentBuilder::new()
            .tools(&[move_action])
            .build(llm)
            .unwrap();

    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn move_up(&mut self, map: &Map) -> bool {
        self.move_to(self.x, self.y - 1, map)
    }

    fn move_down(&mut self, map: &Map) -> bool {
        self.move_to(self.x, self.y + 1, map)
    }

    fn move_left(&mut self, map: &Map) -> bool {
        self.move_to(self.x - 1, self.y, map)
    }

    fn move_right(&mut self, map: &Map) -> bool {
        self.move_to(self.x + 1, self.y, map)
    }

    fn translate(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}

impl SimpleAgent {
    fn look_for_objects_around(&self, map: &mut Map) -> Vec<Arc<Mutex<Object>>> {
        let x = self.x;
        let y = self.y;
        let vision_range = self.vision_range;
        let range_x = x as isize - vision_range as isize..=x as isize + vision_range as isize;
        let range_y = y as isize - vision_range as isize..=y as isize + vision_range as isize;

        let objects: Vec<Arc<Mutex<Object>>> = map.objects.iter()
            .filter(|object| {
                let object = object.lock().unwrap();
                range_x.contains(&(object.x as isize)) && range_y.contains(&(object.y as isize))
            })
            .map(|object| Arc::clone(object))
            .collect();
        objects
    }
    
    fn look_around(&self, map: &Map) -> Vec<Arc<Mutex<Cell>>> {
        let x = self.x;
        let y = self.y;
        let vision_range = self.vision_range;
        let range_x = x as isize - vision_range as isize..=x as isize + vision_range as isize;
        let range_y = y as isize - vision_range as isize..=y as isize + vision_range as isize;

        let cells: Vec<Arc<Mutex<Cell>>> = map.cells.iter()
            .filter(|object| {
                if let Ok(object) = object.lock() {
                    range_x.contains(&(object.x as isize)) && range_y.contains(&(object.y as isize))
                } else {
                    false
                }
            })
            .map(|object| Arc::clone(object))
            .collect();
        cells
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utility::test::create_temp_file;

    fn create_test_map() -> Map {
        let file_content = r#"
        {
            "width": 10,
            "height": 10,
            "cells": [{"id": 0, "description": "grass", "passable": true}, {"id": 2, "description": "water", "passable": false}],
            "objects": [{"name": "tree", "description": "oak tree", "actions": [
                {"name": "chop", "description": "cut down the tree", "requirements": ["axe"]}
            ]}],
            "map": [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 2, 2, 0, 0],
                [0, 0, 0, 0, 0, 0, 2, 2, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            ],
            "located_objects": [
                [0, 0, "tree"], 
                [6, 1, "tree"], 
                [6, 2, "tree"], 
                [9, 9, "tree"]
            ]
        }
        "#;
        let file_path = create_temp_file("test_map.json", file_content);

        Map::load_from_file(file_path.to_str().unwrap()).unwrap()
    }
    
    #[test]
    fn agent_moves_to_valid_location() {
        let mut agent = SimpleAgent {
            name: "TestAgent".to_string(),
            x: 5,
            y: 2,
            energy: 100,
            vision_range: 2,
            knowledge: "".to_string(),
            memory: "".to_string(),
            self_agent: Arc::new(Mutex::new(None))
        };
        let map = create_test_map();
        assert!(agent.move_to(5, 3, &map));
        assert_eq!(agent.x, 5);
        assert_eq!(agent.y, 3);
    }

    #[test]
    fn agent_does_not_move_to_invalid_location() {
        let mut agent = SimpleAgent {
            name: "TestAgent".to_string(),
            x: 5,
            y: 2,
            energy: 100,
            vision_range: 5,
            knowledge: "".to_string(),
            memory: "".to_string(),
        };
        let map = create_test_map();
        assert!(!agent.move_to(6, 2, &map));
        assert_eq!(agent.x, 5);
        assert_eq!(agent.y, 2);
    }

    #[test]
    fn agent_looks_for_objects_in_vision_range() {
        let agent = SimpleAgent {
            name: "TestAgent".to_string(),
            x: 5,
            y: 2,
            energy: 100,
            vision_range: 2,
            knowledge: "".to_string(),
            memory: "".to_string(),
        };
        let mut map = create_test_map();
        let objects = agent.look_for_objects_around(&mut map);
        assert_eq!(objects.len(), 2);
    }

    #[test]
    fn agent_looks_around_in_vision_range() {
        let agent = SimpleAgent {
            name: "TestAgent".to_string(),
            x: 5,
            y: 5,
            energy: 100,
            vision_range: 2,
            knowledge: "".to_string(),
            memory: "".to_string(),
        };
        let map = create_test_map();
        let cells = agent.look_around(&map);
        assert_eq!(cells.len(), 25); // 5x5 grid around the agent
    }
}