//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // Add the camera rotation system to be updated every frame
        .add_systems(Update, camera_rotation_system)
        .run();

    Ok(())
}
/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}


// This system rotates the camera around the scene
pub fn camera_rotation_system(mut query: Query<(&mut Transform, &Camera3d), With<Camera3d>>, time: Res<Time>) {
    // The speed of the rotation
    let rotation_speed = 0.5; // radians per second
    for (mut transform, _) in query.iter_mut() {
        // Calculate the angle based on the time the app has been running
        let angle = time.elapsed_seconds() * rotation_speed;
        // Calculate the new position of the camera
        let x = angle.cos() * 10.0; // Distance from the center
        let z = angle.sin() * 10.0;
        // Update the transform to rotate around the scene
        *transform = Transform::from_xyz(x, 4.5, z).looking_at(Vec3::ZERO, Vec3::Y);
    }
}
