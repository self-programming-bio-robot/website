use std::cell::RefCell;
use std::rc::Rc;

use gloo::console::log;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, HtmlElement};
use yew::{function_component, Html, html, Callback, use_memo, use_state};
use zhdanov_wire_world::world::{World, Point, CellChange, CellType};
use crate::components::canvas::{Canvas, WithRander};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone)]
pub struct Rander {
    counter: usize,
    world: World,
}

impl PartialEq for Rander {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

impl Rander {
    
    fn new(width: usize, height: usize) -> Rander {
        let mut world = World::new(width, height);
        world.add_cell(Point(4, 2), CellType::ELECTRON);
        world.add_cell(Point(4, 3), CellType::WIRE);
        world.add_cell(Point(4, 4), CellType::WIRE);
        world.add_cell(Point(4, 5), CellType::WIRE);
        world.add_cell(Point(4, 6), CellType::WIRE);
        world.add_cell(Point(4, 7), CellType::WIRE);
        world.add_cell(Point(3, 8), CellType::WIRE);
        world.add_cell(Point(5, 8), CellType::WIRE);
        world.add_cell(Point(3, 9), CellType::WIRE);
        world.add_cell(Point(5, 9), CellType::WIRE);
        world.add_cell(Point(4, 9), CellType::WIRE);
        world.add_cell(Point(4, 10), CellType::WIRE);
        world.add_cell(Point(4, 11), CellType::WIRE);
        world.add_cell(Point(4, 12), CellType::WIRE);
        
        world.add_cell(Point(8, 2), CellType::ELECTRON);
        world.add_cell(Point(8, 3), CellType::WIRE);
        world.add_cell(Point(8, 4), CellType::WIRE);
        world.add_cell(Point(8, 5), CellType::WIRE);
        world.add_cell(Point(8, 6), CellType::WIRE);
        world.add_cell(Point(8, 7), CellType::WIRE);
        world.add_cell(Point(7, 8), CellType::WIRE);
        world.add_cell(Point(9, 8), CellType::WIRE);
        world.add_cell(Point(7, 7), CellType::WIRE);
        world.add_cell(Point(9, 7), CellType::WIRE);
        world.add_cell(Point(8, 9), CellType::WIRE);
        world.add_cell(Point(8, 10), CellType::WIRE);
        world.add_cell(Point(8, 11), CellType::WIRE);
        world.add_cell(Point(8, 12), CellType::WIRE);
        
        Rander { counter: 0, world }
    }

    fn click_point(&mut self, x: i32, y:i32) {
        log!(format!("x: {} y: {}", x, y));

        let cell_size = 20.0;
        let r = ((y as f64) / cell_size).floor() as usize;
        let c = ((x as f64) / cell_size).floor() as usize;

        log!(format!("r: {} c: {}", r, c));
        log!("counter", self.counter);
        self.world.add_cell(Point(r, c), CellType::ELECTRON);
        log!("added");
    }
    
    fn draw_cell(&self, cell: &CellChange, context: &CanvasRenderingContext2d) {
        let color = match cell.new_state {
            CellType::EMPTY => JsValue::from_str("black"),
            CellType::WIRE => JsValue::from_str("gray"),
            CellType::ELECTRON => JsValue::from_str("yellow"),
            CellType::TAIL => JsValue::from_str("green"),
        };
        context.set_fill_style(&color);

        let cell_size = 20.0;
        let Point(r, c) = cell.position;
        context.fill_rect(
            (c as f64) * cell_size,
            (r as f64) * cell_size, 
            cell_size, 
            cell_size
        );
    }
}

impl WithRander for Rander {
    fn rand(&mut self, canvas: &HtmlCanvasElement) {
        self.counter += 1;
        let interface: CanvasRenderingContext2d = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        //interface.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        if self.counter % 3 == 0 {
            self.world.add_cell(Point(4, 2), CellType::ELECTRON);
        }
        let changes = self.world.tick();
        for change in changes {
            self.draw_cell(&change, &interface);
        }
    }
}

#[function_component(WireWorld)]
pub fn wire_world() -> Html {
    let mut wireworld = Rc::new(RefCell::from(Rander::new(20, 20)));
    let onclick = {
        let wireworld = wireworld.clone();
        Callback::from(move |event: MouseEvent| {
            let wireworld = wireworld.as_ref();
            let mut  wireworld = wireworld.borrow_mut();
            let x = event.x();
            let y = event.y();
            wireworld.click_point(x, y);
        })
    };

    let wireworld = wireworld.clone();
    html!(
        <Canvas<CanvasRenderingContext2d, Rander>
                style="
                    width: 100vw;
                    height: 100vh;
                "
                rander={wireworld}
                onclick={onclick}
        >
                {"The browser is not supported."}
        </Canvas<CanvasRenderingContext2d, Rander>>
    )
}
