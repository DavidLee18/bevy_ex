use bevy::prelude::*;
use bevy_flycam::prelude::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(NoCameraPlayerPlugin)
    .add_startup_systems((setup_cam, spawn_cubes))
    .run();
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((Camera3dBundle::default(), FlyCam));
}

fn spawn_cubes(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mesh = mesh_assets.add(shape::Box::new(1.0, 1.0, 1.0).into());

    for x in -10..10 {
        for z in -10..10 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                transform: Transform::from_xyz(x as f32 * 2.0, 0.0, z as f32 * 2.0),
                ..Default::default()
            });
        }
    }
}