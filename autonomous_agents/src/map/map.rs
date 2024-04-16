use std::cell::{OnceCell, RefCell};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use serde::Deserialize;
use crate::map::cell::{Cell, CellDefinition};
use crate::map::object::{Object, ObjectDefinition};

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Rc<OnceCell<Cell>>>,
    pub objects: Vec<Rc<RefCell<Object>>>,
    pub object_definitions: HashMap<String, ObjectDefinition>,
}

#[derive(Deserialize)]
struct MapFile {
    width: usize,
    height: usize,
    cells: Vec<CellDefinition>,
    objects: Vec<ObjectDefinition>,
    map: Vec<Vec<usize>>,
    located_objects: Vec<(usize, usize, String)>,
}

impl Map {
    pub fn load_from_file(file_path: &str) -> Result<Map, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(file_path)?;
        let map_file: MapFile = serde_json::from_str(&file_content)?;
        let cell_definitions: HashMap<usize, CellDefinition> = map_file.cells
            .into_iter().map(|it| (it.id, it)).collect();
        let cell_definitions = &cell_definitions; 
        
        let cells: Vec<Rc<OnceCell<Cell>>> = map_file.map.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, cell_id)| {
                let cell_definition = cell_definitions.get(cell_id).unwrap();
                let cell = Cell {
                    id: *cell_id,
                    x,
                    y,
                    description: cell_definition.description.clone(),
                    passable: cell_definition.passable,
                };
                Rc::new(OnceCell::from(cell))
            })
        }).collect();
        let object_definitions: HashMap<String, ObjectDefinition> = map_file.objects
            .into_iter().map(|x| (x.name.clone(), x)).collect();
        let objects: Vec<Rc<RefCell<Object>>> = map_file.located_objects.iter().map(|(x, y, name)| {
            let obj = Object {
                x: *x,
                y: *y,
                data: object_definitions.get(name).unwrap().clone(),
            };
            Rc::new(RefCell::new(obj))
        }).collect();
        Ok(Map {
            width: map_file.width,
            height: map_file.height,
            cells,
            objects,
            object_definitions,
        })
    }
    
    pub fn get_cell(&self, x: usize, y: usize) -> Option<Rc<OnceCell<Cell>>> {
        let index = y * self.width + x;
        self.cells.get(index).map(|cell| Rc::clone(cell))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utility::test::create_temp_file;
    
    #[test]
    fn load_from_file_loads_valid_map() {
        let file_content = r#"
        {
            "width": 2,
            "height": 2,
            "cells": [{"id": 0, "description": "grass", "passable": true}, {"id": 2, "description": "water", "passable": false}],
            "objects": [{"name": "tree", "description": "oak tree", "actions": [
                {"name": "chop", "description": "cut down the tree", "requirements": ["axe"]}
            ]}],
            "map": [[0, 2], [2, 0]],
            "located_objects": [[0, 0, "tree"]]
        }
        "#;
        let file_path = create_temp_file("test_map.json", file_content);

        let map = Map::load_from_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(map.width, 2);
        assert_eq!(map.height, 2);
        assert_eq!(map.cells.len(), 4);
        assert_eq!(map.objects.len(), 1);
        assert_eq!(map.object_definitions.len(), 1);
    }

    #[test]
    fn load_from_file_handles_missing_file() {
        let result = Map::load_from_file("/non/existent/path");

        assert!(result.is_err());
    }

    #[test]
    fn load_from_file_handles_invalid_json() {
        let file_content = "not a json";
        let file_path = create_temp_file("test_invalid_map.json", file_content);

        let result = Map::load_from_file(file_path.to_str().unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn load_from_file_handles_incomplete_map() {
        let file_content = r#"
        {
            "width": 2,
            "height": 2,
            "cells": [{"description": "grass"}, {"description": "water"}],
            "objects": [{"name": "tree", "description": "oak tree"}],
            "map": [[0, 1], [1]]
        }
        "#;
        let file_path = create_temp_file("test_incomplete_map.json", file_content);

        let result = Map::load_from_file(file_path.to_str().unwrap());

        assert!(result.is_err());
    }
}