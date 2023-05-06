use std::collections::HashMap;


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Point(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone)]
pub enum CellType {
    EMPTY,
    WIRE,
    ELECTRON,
    TAIL
}

#[derive(Debug, Clone)]
pub struct World {
    size: (usize, usize),
    next_step: Vec<Point>,
    map: Vec<CellType>,
}

#[derive(Debug, Clone)]
pub struct CellChange {
    pub position: Point,
    pub old_state: CellType,
    pub new_state: CellType,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub position: Point,
    pub cell_type: CellType,
}

impl World {
    
    pub fn new(width: usize, height: usize) -> World {
        World { 
            size: (width, height), 
            next_step: Vec::new(), 
            map: vec![CellType::EMPTY; (width * height).into()] 
        }
    }

    pub fn add_cell(&mut self, pos: Point, cell_type: CellType) {
        if cell_type == CellType::ELECTRON || cell_type == CellType::TAIL {
            self.next_step.push(pos.clone());
        }
        let ind = self.index(&pos);
        self.map[ind] = cell_type;
    }

    pub fn remove_cell(&mut self, pos: Point) {
        let ind = self.index(&pos);
        self.map[ind] = CellType::EMPTY; 
    }

    pub fn get_cells(&self) -> Vec<Cell> {
        let cells = self.map.iter().enumerate().map(|(i, cell_type)| {
            Cell {
                position: Point(i / self.size.0, i % self.size.0),
                cell_type: cell_type.clone(),
            }
        }).collect::<Vec<Cell>>();
        cells
    }

    pub fn tick(&mut self) -> Vec<CellChange> {
        let mut next_step: Vec<Point> = Vec::new();
        let mut changes = Vec::new();
        let mut potential_points: HashMap<Point, usize> = HashMap::new();
        for cell in self.next_step.iter() {
            let old_state = self.map[self.index(cell)].clone(); 
            match old_state {
                CellType::ELECTRON => {
                    changes.push(CellChange { 
                        position: cell.clone(), 
                        old_state, 
                        new_state: CellType::TAIL 
                    });

                    let neighbors = self.get_cells_around(cell, &CellType::WIRE);
                    for neighbor in neighbors  {
                        let point = potential_points.entry(neighbor).or_insert(0); 
                        *point += 1;
                    }
                },
                CellType::TAIL => {
                    changes.push(CellChange { 
                        position: cell.clone(), 
                        old_state, 
                        new_state: CellType::WIRE 
                    });
                },
                _other => (),
            }    
        }

        for (point, count) in potential_points {
            if count == 1 || count == 2 {
                changes.push(CellChange { 
                    position: point, 
                    old_state: CellType::WIRE, 
                    new_state: CellType::ELECTRON 
                });
            }
        }

        for change in changes.iter() {
            let ind = self.index(&change.position);
            self.map[ind] = change.new_state.clone();
            next_step.push(change.position.clone());
        }

        self.next_step = next_step;
        return changes;
    }

    fn index(&self, point: &Point) -> usize {
        point.0 * self.size.0 + point.1
    }

    fn get_cells_around(&self, point: &Point, cell_type: &CellType) -> Vec<Point> {
        let mut found = Vec::new();
        let offsets = [(-1, -1), (-1, 0), (-1, 1), 
                        (0, -1), (0, 1), (1, -1), 
                        (1, 0), (1, 1)];

        for offset in offsets.iter() {
            let pos = Point(
                ((point.0 as isize + offset.0 + self.size.1 as isize) as usize) % self.size.1,
                ((point.1 as isize + offset.1 + self.size.0 as isize) as usize) % self.size.0
            );

            let ind = self.index(&pos);
            if cell_type.eq(&self.map[ind]) {
                found.push(pos);
            }
        }

        found
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
         let world = World::new(10, 10);
         assert_eq!(world.size, (10, 10));
         assert_eq!(world.next_step.len(), 0);
         assert_eq!(world.map.len(), 100);
    }

    #[test]
    fn test_add_cell() {
         let mut world = World::new(10, 10);
         world.add_cell(Point(0, 0), CellType::WIRE);
         assert_eq!(world.map[0], CellType::WIRE);
         world.add_cell(Point(0, 1), CellType::ELECTRON);
         assert_eq!(world.map[1], CellType::ELECTRON);
         world.add_cell(Point(0, 2), CellType::TAIL);
         assert_eq!(world.map[2], CellType::TAIL);
    }

    #[test]
    fn test_remove_cell() {
         let mut world = World::new(10, 10);
         world.add_cell(Point(0, 0), CellType::WIRE);
         world.remove_cell(Point(0, 0));
         assert_eq!(world.map[0], CellType::EMPTY);
    }

    #[test]
    fn test_get_cells() {
         let mut world = World::new(10, 10);
         world.add_cell(Point(0, 0), CellType::WIRE);
         world.add_cell(Point(0, 1), CellType::ELECTRON);
         world.add_cell(Point(0, 2), CellType::TAIL);
         assert_eq!(world.get_cells()[0].cell_type, CellType::WIRE);
         assert_eq!(world.get_cells()[1].cell_type, CellType::ELECTRON);
         assert_eq!(world.get_cells()[2].cell_type, CellType::TAIL);
    }

    #[test]
    fn test_delight_elector_signal() {
        let mut world = World::new(1, 1);
        world.add_cell(Point(0, 0), CellType::ELECTRON);
        let changes = world.tick();
        let change = &changes[0];
        assert_eq!(change.position, Point(0,0));
        assert_eq!(change.old_state, CellType::ELECTRON);
        assert_eq!(change.new_state, CellType::TAIL);
        assert_eq!(world.get_cells()[0].cell_type, CellType::TAIL);
    }
    
    #[test]
    fn test_lose_signal() {
        let mut world = World::new(1, 1);
        world.add_cell(Point(0, 0), CellType::TAIL);
        let changes = world.tick();
        let change = &changes[0];
        assert_eq!(change.position, Point(0,0));
        assert_eq!(change.old_state, CellType::TAIL);
        assert_eq!(change.new_state, CellType::WIRE);
        assert_eq!(world.get_cells()[0].cell_type, CellType::WIRE);
    }

    #[test]
    fn test_create_signal() {
        let mut world = World::new(4, 7);
        world.add_cell(Point(0, 0), CellType::WIRE);
        world.add_cell(Point(0, 1), CellType::ELECTRON);
        world.add_cell(Point(2, 0), CellType::ELECTRON);
        world.add_cell(Point(2, 1), CellType::WIRE);
        world.add_cell(Point(2, 2), CellType::ELECTRON);
        world.add_cell(Point(4, 0), CellType::ELECTRON);
        world.add_cell(Point(4, 1), CellType::WIRE);
        world.add_cell(Point(4, 2), CellType::ELECTRON);
        world.add_cell(Point(5, 1), CellType::ELECTRON);
       
        let changes = world.tick();
        
        for change in changes {
            match change {
                CellChange { position: Point(0,0), old_state:_, new_state } => 
                    assert_eq!(new_state, CellType::ELECTRON),
                CellChange { position: Point(2,1), old_state:_, new_state } => 
                    assert_eq!(new_state, CellType::ELECTRON),
                CellChange { position: Point(4,1), old_state:_, new_state } => 
                    assert_eq!(new_state, CellType::WIRE),
                _other => ()
            }
        }
    }

    #[test]
    fn test_index() {
        let world = World::new(4, 7);

        let mut test_case = 0;
        for j in 0..7 {
            for i in 0..4 {
                let ind = world.index(dbg!(&Point(j, i)));
                assert_eq!(test_case, ind);
                test_case += 1;
            }
        }
    }
    
    #[test]
    fn test_get_neighbors() {
        let world = World::new(4, 4);
        let neighbors = world.get_cells_around(&Point(0, 0), &CellType::EMPTY);
        assert_eq!(8, neighbors.len());
    }
}
