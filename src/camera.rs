use crate::{character::Character, input::GeneralInput};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                PostUpdate,
                camera_following
                    .after(PhysicsSet::Sync)
                    .before(TransformSystem::TransformPropagate),
            )
            .init_resource::<MainCameraState>();
    }
}

#[derive(Resource)]
pub struct MainCameraState {
    pub focus: Entity,
    focus_speed: f32,
    distance: f32,
    max_distance: f32,
    min_distance: f32,
    pitch: f32,
    yaw: f32,
    pan_sensitivity: Vec2,
    zoom_sensitivity: f32,
}

impl Default for MainCameraState {
    fn default() -> Self {
        Self {
            focus: Entity::PLACEHOLDER,
            focus_speed: 5.0,
            distance: 5.0,
            max_distance: 10.0,
            min_distance: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            pan_sensitivity: Vec2::new(0.3, 0.1),
            zoom_sensitivity: 10.0,
        }
    }
}

impl MainCameraState {
    pub fn yaw(&self) -> f32 {
        self.yaw
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera3d::default(), MainCamera));
}

// New system to handle camera orbiting
fn camera_following(
    mut cameras: Query<&mut Transform, With<MainCamera>>,
    mut characters: Query<(&Character, &Transform), Without<MainCamera>>,
    mut camera_state: ResMut<MainCameraState>,
    action_state: Res<ActionState<GeneralInput>>,
    time: Res<Time>,
) {
    if let Ok((character, character_transform)) = characters.get_mut(camera_state.focus) {
        // Get entity data, and if either line panics, we have invalid component configuration
        let mut camera_transform = cameras.single_mut();

        // Get action data
        let pan = action_state.axis_pair(&GeneralInput::PanCamera);
        let zoom = action_state.value(&GeneralInput::Zoom);

        // Update yaw, pitch, and distance based on mouse movement and scroll
        camera_state.yaw -= pan.x * camera_state.pan_sensitivity.x * time.delta_secs();
        camera_state.pitch += pan.y * camera_state.pan_sensitivity.y * time.delta_secs();
        camera_state.distance *= 1. - zoom * camera_state.zoom_sensitivity * time.delta_secs();

        // Clamp pitch to prevent camera flipping
        camera_state.pitch = camera_state.pitch.clamp(-1.0, 1.0);

        // Clamp distance to min and max
        camera_state.distance = camera_state
            .distance
            .clamp(camera_state.min_distance, camera_state.max_distance);

        // Calculate new camera position
        let rot = Quat::from_euler(EulerRot::YXZ, camera_state.yaw, camera_state.pitch, 0.0);
        let zoom_offset = rot * Vec3::new(0.0, 0.0, camera_state.distance);
        let head_position =
            character_transform.translation + Vec3::new(0.0, character.height(), 0.0);
        let target_position = head_position + zoom_offset;
        let focus_step = camera_state.focus_speed * time.delta_secs();
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_position, focus_step);
        camera_transform.look_at(head_position, Vec3::Y);

        // TODO: Fix camera clipping through walls
    }
}
