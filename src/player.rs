use bevy::prelude::*;

pub struct PlayerPlugin;

const PLAYER_SPEED: f32 = 200.0;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, move_player);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct LocallyControlled;

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player,
        LocallyControlled,
        Sprite::from_color(Color::srgb(0.2, 0.7, 1.0), Vec2::new(32.0, 32.0)),
    ));
}

fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut players: Query<&mut Transform, (With<Player>, With<LocallyControlled>)>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    let movement = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_secs();
    for mut transform in &mut players {
        transform.translation += movement.extend(0.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn displacement(keys: &[KeyCode], seconds: f32) -> Vec3 {
        let mut app = App::new();
        let mut keyboard = ButtonInput::<KeyCode>::default();
        for &key in keys {
            keyboard.press(key);
        }
        app.insert_resource(keyboard)
            .init_resource::<Time>()
            .add_systems(Update, move_player);
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(seconds));
        let player = app
            .world_mut()
            .spawn((Player, LocallyControlled, Transform::default()))
            .id();
        let remote_player = app.world_mut().spawn((Player, Transform::default())).id();
        let other = app.world_mut().spawn(Transform::default()).id();
        let non_player = app
            .world_mut()
            .spawn((LocallyControlled, Transform::default()))
            .id();
        app.update();
        for entity in [remote_player, non_player] {
            assert_eq!(
                app.world().get::<Transform>(entity).unwrap().translation,
                Vec3::ZERO
            );
        }
        assert_eq!(
            app.world().get::<Transform>(other).unwrap().translation,
            Vec3::ZERO
        );
        app.world().get::<Transform>(player).unwrap().translation
    }

    #[test]
    fn movement_respects_input_speed_and_elapsed_time() {
        assert_eq!(displacement(&[], 0.5), Vec3::ZERO);
        assert_eq!(
            displacement(&[KeyCode::KeyW, KeyCode::KeyS], 0.5),
            Vec3::ZERO
        );
        let straight = displacement(&[KeyCode::KeyD], 0.5);
        assert_eq!(straight, Vec3::new(100.0, 0.0, 0.0));
        let diagonal = displacement(&[KeyCode::KeyD, KeyCode::KeyW], 0.5);
        assert!((diagonal.length() - straight.length()).abs() < 0.001);
        assert!(diagonal.x > 0.0 && diagonal.y > 0.0);
        assert_eq!(displacement(&[KeyCode::KeyD], 0.25) * 2.0, straight);
    }
}
