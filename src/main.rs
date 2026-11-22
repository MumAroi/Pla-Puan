use bevy::prelude::*;

mod camera;
mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pla-Puan".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}
