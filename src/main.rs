use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

fn hello_world() {
    print!("hello world.");
}
