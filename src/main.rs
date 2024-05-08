//! A simple 3D scene with light shining over a cube sitting on a plane.
mod lib;
use bevy::prelude::*;
use lib::{camera_rotation_system, first_person_camera_system, load_gltf, setup};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins))
        .add_systems(Startup, load_gltf)
        .add_systems(Startup, setup)
        .add_systems(Update, first_person_camera_system)
        // Add the camera rotation system to be updated every frame
        .run();
}
