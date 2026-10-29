use avian2d::prelude::*;
use bevy::prelude::*;
use rand::RngExt;

use crate::points::GoalEvent;

const BALL_SIZE: f32 = 6.0;
const BALL_MIN_SPEED_X: f32 = 220.0;
const BALL_WAIT: f32 = 2.0;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallState>()
            .insert_resource(KickCooldown {
                timer: Timer::from_seconds(BALL_WAIT, TimerMode::Once),
            })
            .add_systems(Startup, spawn_ball)
            .add_systems(FixedUpdate, (limit_balls_speed, update_kick_timer))
            .add_observer(kick_ball)
            .add_observer(on_spawn_ball)
            .add_observer(reset_on_goal);
    }
}

#[derive(Bundle)]
pub struct BallBundle {
    sprite: Sprite,
    ball: Ball,
    rb: RigidBody,
    locked_rotation: LockedAxes,
    restitution: Restitution,
    collider: Collider,
    no_friction: Friction,
}
impl BallBundle {
    pub fn new(
        asset_server: Res<AssetServer>,
        mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture: Handle<Image> = asset_server.load("sprites/pong-spritesheet.png");
        let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(32),
            1,
            1,
            None,
            Some(UVec2 { x: 32, y: 0 }),
        ));
        BallBundle {
            sprite: Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 1 }),
            ball: Ball,
            rb: RigidBody::Dynamic,
            locked_rotation: LockedAxes::ROTATION_LOCKED,
            restitution: Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
            collider: Collider::circle(BALL_SIZE),
            no_friction: Friction::ZERO,
        }
    }
}

#[derive(Component)]
pub struct Ball;

#[derive(Resource, PartialEq, Default, Debug)]
enum BallState {
    #[default]
    IDLE,
    MOVING,
}

#[derive(Resource, Default)]
pub struct KickCooldown {
    timer: Timer,
}

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(BallBundle::new(asset_server, atlas_layouts));
}

#[derive(Event)]
struct KickBallEvent;
fn kick_ball(
    _event: On<KickBallEvent>,
    ball: Single<&mut LinearVelocity, With<Ball>>,
    mut ball_state: ResMut<BallState>,
) {
    let mut ball_vel = ball.into_inner();
    let mut rng = rand::rng();
    let rand_dir = Vec2::new(rng.random_range(-5.0..5.0), rng.random_range(-1.0..1.0)).normalize();

    ball_vel.x = rand_dir.x * BALL_MIN_SPEED_X;
    ball_vel.y = rand_dir.y * BALL_MIN_SPEED_X;
    *ball_state = BallState::MOVING;
}

fn limit_balls_speed(ball: Single<&mut LinearVelocity, With<Ball>>, ball_state: Res<BallState>) {
    if *ball_state == BallState::IDLE {
        return;
    }

    // CONSTRAINT BALL SPEED
    let mut ball_vel = ball.into_inner();

    if ball_vel.x.abs() > BALL_MIN_SPEED_X {
        return;
    }

    ball_vel.x = ball_vel.x.signum() * BALL_MIN_SPEED_X;
}

fn on_spawn_ball(event: On<Insert<Ball>>, mut ball_state: ResMut<BallState>) {
    // reset idle timer
    *ball_state = BallState::IDLE;

    let new_ball = event.entity;
    println!("{}", new_ball);
}

fn update_kick_timer(
    mut commands: Commands,
    mut kick_cooldown: ResMut<KickCooldown>,
    global_timer: Res<Time>,
) {
    if kick_cooldown.timer.is_finished() {
        return;
    }

    kick_cooldown.timer.tick(global_timer.delta());

    if kick_cooldown.timer.is_finished() {
        commands.trigger(KickBallEvent);
    }
}

fn reset_on_goal(
    _event: On<GoalEvent>,
    mut kick_cooldown: ResMut<KickCooldown>,
    mut ball_state: ResMut<BallState>,
) {
    kick_cooldown.timer.reset();
    *ball_state = BallState::IDLE;
}
