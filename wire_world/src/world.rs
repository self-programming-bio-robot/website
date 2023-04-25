
#[derive(Debug, PartialEq)]
pub struct Point(usize, usize);

#[derive(Debug, PartialEq)]
pub enum CellType {
    EMPTY,
    WIRE,
    ELECTRON,
    TAIL
}

#[derive(Debug)]
pub struct World {
    size: (usize, usize),
    next_step: Vec<Point>,
    map: Vec<CellType>,
}

#[derive(Debug)]
pub struct CellChange {
    pub position: Point,
    pub old_state: CellType,
    pub new_state: CellType,
}

impl World {
    
    pub fn new(width: usize, height: usize) -> World {
        todo!()
    }

    pub fn add_cell(&mut self, pos: Point, cell_type: CellType) {
        todo!()
    }

    pub fn remove_cell(&mut self, pos: Point) {
        todo!()
    }

    pub fn get_cells(&self) -> Vec<CellType> {
        todo!()
    }

    pub fn tick(&mut self) -> Vec<CellChange> {
        todo!()
    }
}

#[cfg(test)]
mod Test {
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
         assert_eq!(world.get_cells()[0], CellType::WIRE);
         assert_eq!(world.get_cells()[1], CellType::ELECTRON);
         assert_eq!(world.get_cells()[2], CellType::TAIL);
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
        assert_eq!(world.get_cells()[0], CellType::TAIL);
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
        assert_eq!(world.get_cells()[0], CellType::WIRE); 
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
}
