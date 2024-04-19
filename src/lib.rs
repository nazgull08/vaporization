//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::prelude::*;
use bevy_gltf::{Gltf, GltfMesh};
use wasm_bindgen::prelude::*;

/// Helper resource for tracking our asset
#[derive(Resource)]
pub struct MyAssetPack(Handle<Gltf>);


#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {

    App::new()
        .add_plugins((DefaultPlugins))
        .add_systems(Startup, load_gltf)
        .add_systems(Startup, setup)
        // Add the camera rotation system to be updated every frame
        .add_systems(Update, camera_rotation_system)
        .run();

    Ok(())
}

fn load_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
) {
    let gltf = ass.load("glb/skull.glb");
    commands.insert_resource(MyAssetPack(gltf));
    info!("=====Loaded own glbs")
}

/// set up a simple 3D scene
pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets_gltf: Res<Assets<Gltf>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    ass: Res<AssetServer>,
) {
    let skull_gltf = ass.load("glb/skull.glb#Scene0");
    let ram_gltf = ass.load("glb/ram.glb#Scene0");
    let sheep_gltf = ass.load("glb/sheep.glb#Scene0");
    //commands.insert_resource(MyAssetPack(gltf));
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    /*
    if let Some(gltf) = assets_gltf.get(&my.0) {
        // Get the GLTF Mesh named "CarWheel"
        // (unwrap safety: we know the GLTF has loaded already)
        let skull = assets_gltfmesh.get(&gltf.named_meshes["skull"]).unwrap();

        // Spawn a PBR entity with the mesh and material of the first GLTF Primitive
        commands.spawn(PbrBundle {
            mesh: skull.primitives[0].mesh.clone(),
            // (unwrap: material is optional, we assume this primitive has one)
            material: skull.primitives[0].material.clone().unwrap(),
            ..Default::default()
        });
    }*/
    
    commands.spawn(SceneBundle {
        scene: skull_gltf,
        transform: Transform::from_xyz(0.5, 0.0, -1.0),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: ram_gltf,
        transform: Transform::from_xyz(1.5, 0.0, -5.0),
        ..Default::default()
    });

    commands.spawn(SceneBundle {
        scene: sheep_gltf,
        transform: Transform::from_xyz(-1.5, 0.0, -3.0),
        ..Default::default()
    });
    
    /*
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        mesh: gltf,
        material: materials.add(Color::BLUE),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    */
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
