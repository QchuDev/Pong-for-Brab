use avian2d::prelude::*;
use bevy::prelude::*;

const BALL_SIZE: f32 = 6.0;
const BALL_SPEED: f32 = 300.0;
const BALL_MIN_SPEED_X: f32 = 300.0;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_ball, kick_ball).chain())
            .add_systems(FixedUpdate, check_ball);
    }
}

#[derive(Component)]
pub struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("sprites/pong-spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        1,
        1,
        None,
        Some(UVec2 { x: 32, y: 0 }),
    ));

    let ball_sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 1 });

    // Spawn Ball
    commands.spawn((
        ball_sprite,
        Ball,
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Restitution::new(1.0).with_combine_rule(CoefficientCombine::Max),
        Collider::circle(BALL_SIZE),
    ));
}

fn kick_ball(ball: Single<&mut LinearVelocity, With<Ball>>) {
    let mut ball_vel = ball.into_inner();
    let rand_dir = rand::random::<Vec2>().normalize();
    ball_vel.x = rand_dir.x * BALL_SPEED;
    ball_vel.y = rand_dir.y * BALL_SPEED;
}

fn check_ball(ball: Single<&mut LinearVelocity, With<Ball>>) {
    // CONSTRAINT BALL SPEED
    let mut ball_vel = ball.into_inner();

    if ball_vel.x.abs() > BALL_MIN_SPEED_X {
        return;
    }

    ball_vel.x = ball_vel.x.signum() * BALL_MIN_SPEED_X;
}
