use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
mod pad;
use pad::*;

mod player;
use player::*;

mod kinematics;
use kinematics::*;

mod ball;
use ball::*;

// We will make a pong for my neural network managmente library - Brab
//
// By using Bevy we have a game engine for the pong.
//
// REQUIREMENTS OF THIS PONG:
// - Simple and Nice looking
// - Modular, each part of the system should be re-usable for future projects
// - Playable for the user
// - The agent shoudl learn by 'Neuroevolution'
// - Feedback is needed: points and rewards
//
// PLAN:
// - Make simple movement  and collision with AABB
// - Move of platform
// - Points
// - Assets
//
// Just that, simple, it should take a week only

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgba(0.2, 0.2, 0.2, 1.0)))
        .add_plugins(PongPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

// The PongPlugin
struct PongPlugin;
impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "testing".into(),
                        resolution: WindowResolution::new(600, 600),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(BallPlugin)
        .add_plugins(PadPlugin)
        .add_plugins(KinematicsPlugin)
        .add_plugins(PlayerPlugin);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
