use crate::{SCREEN_HEIGHT, SCREEN_WIDTH, ball::Ball};
use avian2d::{
    collision::{
        collider::Collider,
        collision_events::{CollisionEventsEnabled, CollisionStart},
    },
    dynamics::rigid_body::RigidBody,
};
use bevy::prelude::*;
use std::collections::HashMap;

pub struct PointsPlugin;
impl Plugin for PointsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Points>()
            // adds a callback function with an observer associated
            .add_observer(on_goal)
            .add_systems(Startup, (setup_goals, setup_points))
            .add_systems(
                FixedUpdate,
                update_scoreboard.run_if(resource_changed::<Points>), // conditioned
                                                                      // task?
            );
    }
}

#[derive(Component)]
struct Goal;
//
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Team {
    RIGHT,
    LEFT,
}

// Resource for persistency at points
#[derive(Resource, Default)]
pub struct Points {
    board: HashMap<Team, u32>,
}
impl Points {
    pub fn get(&self, team: Team) -> u32 {
        self.board.get(&team).copied().unwrap_or(0)
    }

    pub fn add(&mut self, team: Team) {
        *self.board.entry(team).or_default() += 1;
    }
}

// Systems of the plugin
fn setup_goals(mut commands: Commands) {
    commands.spawn((
        Goal,
        Team::LEFT,
        RigidBody::Static,
        CollisionEventsEnabled,
        Transform::from_xyz(SCREEN_WIDTH as f32 * 0.5, 0.0, 0.0),
        Collider::rectangle(10.0, SCREEN_HEIGHT as f32),
    ));

    commands.spawn((
        Goal,
        Team::RIGHT,
        RigidBody::Static,
        CollisionEventsEnabled,
        Transform::from_xyz(-(SCREEN_WIDTH as f32 * 0.5), 0.0, 0.0),
        Collider::rectangle(10.0, SCREEN_HEIGHT as f32),
    ));
}

fn on_goal(
    ev: On<CollisionStart>,
    goals: Query<&Team, With<Goal>>,
    balls: Query<(), With<Ball>>,
    mut points: ResMut<Points>,
) {
    // Check for collision at goals and sum a point
    let (a, b) = (ev.collider1, ev.collider2);
    let (goal, ball) = if goals.contains(a) { (a, b) } else { (b, a) };

    if let (Ok(team), true) = (goals.get(goal), balls.contains(ball)) {
        points.add(*team);
    }
    println!("{} | {}", points.get(Team::LEFT), points.get(Team::RIGHT));
}

#[derive(Component)]
struct ScoreText(Team);

const SCORE_DISPLACEMENT: f32 = 150.0;
const FONT_SIZE: f32 = 24.0;

fn setup_points(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Create a TextFont
    // Make the scoreboards for each team
    commands.spawn((
        Text2d::new("-"),
        TextFont {
            font_size: FontSize::Px(FONT_SIZE),
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(-SCORE_DISPLACEMENT, SCORE_DISPLACEMENT * 0.5, 0.0),
        ScoreText(Team::LEFT),
    ));
    commands.spawn((
        Text2d::new("-"),
        TextFont {
            font_size: FontSize::Px(FONT_SIZE),
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(SCORE_DISPLACEMENT, SCORE_DISPLACEMENT * 0.5, 0.0),
        ScoreText(Team::RIGHT),
    ));
}

fn update_scoreboard(mut scoreboards: Query<(&mut Text2d, &ScoreText)>, points: ResMut<Points>) {
    for (mut text, score) in &mut scoreboards {
        text.0 = points.get(score.0).to_string();
    }
}
