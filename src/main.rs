mod camera;
mod character;
mod input;

use crate::{camera::CameraPlugin, character::CharacterPlugin, input::InputPlugin};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        WorldInspectorPlugin::new(),
        InputPlugin,
        CameraPlugin,
        CharacterPlugin,
        PhysicsPlugins::default(),
    ))
    .add_systems(Startup, setup);
    app.run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn sun
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Spawn ground
    commands.spawn((
        Mesh3d(meshes.add(Mesh::from(Cylinder::new(10.0, 1.0)))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::cylinder(10.0, 1.0),
    ));
}
