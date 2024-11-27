mod camera;
mod character;
mod input;

use crate::{camera::CameraPlugin, character::CharacterPlugin, input::InputPlugin};
use avian3d::prelude::*;
use bevy::{log::LogPlugin, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.build().disable::<LogPlugin>(),
        WorldInspectorPlugin::new(),
        InputPlugin,
        CameraPlugin,
        CharacterPlugin,
        PhysicsPlugins::default(),
    ))
    .add_systems(Startup, setup);
    app.run();
    //bevy_mod_debugdump::print_schedule_graph(&mut app, PostUpdate);
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn sun
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Spawn ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(Cylinder::new(10.0, 1.0))),
            material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
            transform: Transform::from_xyz(0.0, -1.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::cylinder(10.0, 1.0),
    ));
}
