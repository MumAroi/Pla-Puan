use bevy::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
