use bevy::prelude::*;
use zhdanov_wire_world::GamePlugin;

fn main() {
    App::new()
        .add_plugin(GamePlugin)
        .run();
}