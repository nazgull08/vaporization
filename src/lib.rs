//! A simple 3D scene with light shining over a cube sitting on a plane.
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_gltf::{Gltf, GltfMesh};
use wasm_bindgen::prelude::*;

/// Helper resource for tracking our asset
#[derive(Resource)]
pub struct MyAssetPack(Handle<Gltf>);

#[derive(Debug,Component)]
pub struct FirstPersonCamera {
    pub speed: f32,
    pub sensitivity: f32,
}



#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {

    App::new()
        .add_plugins((DefaultPlugins))
        .add_systems(Startup, load_gltf)
        .add_systems(Startup, setup)
        .add_systems(Update, first_person_camera_system)
        // Add the camera rotation system to be updated every frame
        .run();

    Ok(())
}

pub fn load_gltf(
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

    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    
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
    }).insert(FirstPersonCamera {
        speed: 5.0,
        sensitivity: 0.1,
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

pub fn first_person_camera_system(
    mut query: Query<(&mut Transform, &FirstPersonCamera), With<Camera3d>>,
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
) {
     let (mut transform, camera) = query.single_mut();
    let mut delta_mouse = Vec2::ZERO;

    for event in mouse_motion_events.read() {
        delta_mouse += event.delta;
    }

    // Обрабатываем горизонтальное вращение камеры (yaw)
    transform.rotate_y(-delta_mouse.x * camera.sensitivity * time.delta_seconds());

    // Обрабатываем вертикальное вращение камеры (pitch) с ограничением
    let mut local_rotation = Quat::from_rotation_x(-delta_mouse.y * camera.sensitivity * time.delta_seconds());
    let current_pitch = transform.rotation.to_euler(EulerRot::XYZ).0; // Получаем текущий pitch
    let new_pitch = current_pitch + local_rotation.to_euler(EulerRot::XYZ).0;

    // Ограничиваем pitch чтобы избежать переворота камеры
    if new_pitch > -1.5 && new_pitch < 1.5 {
        transform.rotation = transform.rotation * local_rotation;
    }

    // Нормализуем угол поворота камеры
    transform.rotation = transform.rotation.normalize();

    // Обрабатываем движение камеры
    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        direction += transform.forward().normalize();
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        direction -= transform.forward().normalize();
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        direction -= transform.right().normalize();
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        direction += transform.right().normalize();
    }

    // Двигаем камеру с учетом скорости
    if direction.length_squared() > 0.0 {
        transform.translation += direction.normalize() * camera.speed * time.delta_seconds();
    }
}
