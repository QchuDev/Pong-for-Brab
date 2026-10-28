use avian2d::prelude::*;
use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
mod pad;
use pad::*;

mod player;
use player::*;

mod bot;
use bot::*;

mod ball;
use ball::*;

mod points;
use points::*;

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
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

fn main() {
    App::new()
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .insert_resource(ClearColor(Color::srgba(0.2, 0.2, 0.2, 1.0)))
        .add_plugins(PongPlugin)
        .run();
}

// The PongPlugin
struct PongPlugin;
impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_plugins(
                DefaultPlugins
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "testing".into(),
                            resolution: WindowResolution::new(SCREEN_WIDTH, SCREEN_HEIGHT),
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
            .add_plugins((
                PhysicsPlugins::default(),
                // Enables debug rendering
                PhysicsDebugPlugin,
            ))
            .insert_resource(Gravity::ZERO)
            //
            // PHYSICS GIZMOS
            //
            .insert_gizmo_config(
                PhysicsGizmos {
                    aabb_color: Some(Color::WHITE),
                    ..default()
                },
                GizmoConfig::default(),
            )
            .add_plugins(BallPlugin)
            .add_plugins(PadPlugin)
            .add_plugins(PlayerPlugin)
            .add_plugins(BotPlugin)
            .add_plugins(PointsPlugin);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
    //setup walls
    commands.spawn((
        RigidBody::Static,
        Transform::from_xyz(0.0, SCREEN_HEIGHT as f32 * 0.5, 0.0),
        Collider::rectangle(SCREEN_WIDTH as f32, 10.0),
    ));
    commands.spawn((
        RigidBody::Static,
        Transform::from_xyz(0.0, -(SCREEN_HEIGHT as f32 * 0.5), 0.0),
        Collider::rectangle(SCREEN_WIDTH as f32, 10.0),
    ));
}
