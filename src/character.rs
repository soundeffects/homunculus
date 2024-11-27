use std::f32::consts::PI;

use crate::{camera::MainCameraState, input::UserAction};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_character)
            .add_systems(Update, apply_physics);
    }
}

fn setup_character(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut camera_state: ResMut<MainCameraState>,
) {
    // Spawn character
    let character_id = commands
        .spawn((
            SceneBundle {
                scene: asset_server.load("humanoid.glb#Scene0"),
                transform: Transform::from_xyz(0.0, 1.0, 0.0),
                ..default()
            },
            Character::default(),
            RigidBody::Dynamic,
            Collider::capsule_endpoints(0.2, Vec3::new(0.0, 0.2, 0.0), Vec3::new(0.0, 1.8, 0.0)),
            Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            LockedAxes::new().lock_rotation_x().lock_rotation_z(),
        ))
        .id();

    // Make the camera focus on the character
    camera_state.focus = character_id;
}

fn apply_physics(
    camera_state: ResMut<MainCameraState>,
    mut characters: Query<(&Character, &mut LinearVelocity, &mut AngularVelocity)>,
    action_state: Res<ActionState<UserAction>>,
    time: Res<Time>,
) {
    if let Ok((character, mut linear_velocity, mut angular_velocity)) =
        characters.get_mut(camera_state.focus)
    {
        let pan = action_state.axis_pair(&UserAction::PanCamera);
        let zoom = action_state.value(&UserAction::Zoom);
        let movement = action_state.axis_pair(&UserAction::Move);

        let target_rotation = Quat::from_rotation_y(PI + camera_state.yaw());

        // Smoothly interpolate current rotation to target rotation
        let rotation_step = character.rotation_speed * time.delta_seconds();

        //character_transform.rotation = character_transform
        //    .rotation
        //    .slerp(target_rotation, rotation_step);

        // Move relative to camera's orientation
        let velocity_input = Vec3::new(movement.x, 0.0, movement.y).normalize_or_zero();
        let target_velocity = target_rotation * velocity_input * character.lateral_speed;
        let velocity_step = character.lateral_acceleration * time.delta_seconds();
        linear_velocity.x = linear_velocity.x.lerp(target_velocity.x, velocity_step);
        linear_velocity.z = linear_velocity.z.lerp(target_velocity.z, velocity_step);

        // TODO: Balancing forces to prevent x/z rotation + off balance flag
        // TODO: Lean towards acceleration direction
        // TODO: Jump
        // TODO: Fix jitter when moving
    }
}

#[derive(Component)]
pub struct Character {
    lateral_speed: f32,
    lateral_acceleration: f32,
    rotation_speed: f32,
    height: f32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            lateral_speed: 10.0,
            lateral_acceleration: 10.0,
            rotation_speed: 10.0,
            height: 1.8,
        }
    }
}

impl Character {
    pub fn height(&self) -> f32 {
        self.height
    }
}
