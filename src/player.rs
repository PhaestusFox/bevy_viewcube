use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct LookData {
    yaw: f32,
    pitch: f32,
}

impl Default for LookData {
    fn default() -> Self {
        LookData {
            yaw: 0.,
            pitch: 45.0f32.to_radians(),
        }
    }
}

fn look_mode(input: Res<Input<MouseButton>>) -> bool {
    input.pressed(MouseButton::Middle)
}

fn move_mode(input: Res<Input<MouseButton>>) -> bool {
    input.pressed(MouseButton::Left)
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 1.,
            color: Color::WHITE,
        })
        //.add_system(move_player.run_if(move_mode))
        .add_system(player_look.run_if(look_mode));
    }
}

fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Children), With<Player>>,
    camera: Query<&Transform, Without<Player>>,
    mut mouse_move: EventReader<MouseMotion>,
) {
    let mut player_movement = Vec2::ZERO;
    for MouseMotion { delta } in mouse_move.iter() {
        player_movement += *delta;
    }
    for (mut player_transform, children) in &mut player_query {
        let local_z =
            if let Ok(cam_tran) = camera.get(*children.get(0).unwrap_or(&Entity::from_raw(0))) {
                cam_tran.local_z()
            } else {
                error!("First Child on player should be camera");
                continue;
            };
        let forward = player_movement.y * -Vec3::new(local_z.x, 0., local_z.z)
            + player_movement.x * -Vec3::new(local_z.z, 0., -local_z.x);
        player_transform.translation += forward * time.delta_seconds();
    }
}

fn player_look(
    mut player: Query<(&mut Transform, &mut LookData)>,
    mut mouse_move: EventReader<MouseMotion>,
) {
    let mut total = Vec2::ZERO;
    for MouseMotion { delta } in mouse_move.iter() {
        total += *delta;
    }
    for (mut transfrom, mut data) in player.iter_mut() {
        data.yaw += total.x * 0.001;
        data.pitch += total.y * 0.001;
        let cos = data.yaw.cos();
        let sin = data.yaw.sin();

        transfrom.translation = Vec3::new(cos - sin, data.pitch, cos + sin) * 5.0;
        transfrom.look_at(Vec3::ZERO, Vec3::Y);
    }
}
