use bevy::prelude::*;
use zhdanov_wire_world::GamePlugin;

fn main() {
    App::new()
        .add_plugins(GamePlugin)
        .run();
}