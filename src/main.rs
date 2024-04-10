//! A simple 3D scene with light shining over a cube sitting on a plane.
mod lib;
use bevy::prelude::*;
use lib::{camera_rotation_system, setup};


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_rotation_system)
        .run();
}
