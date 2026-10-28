use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::ball::Ball;
const BOT_MAX_SPEED: f32 = 200.0;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, bot_following);
    }
}

#[derive(Component)]
pub struct Bot;

fn bot_following(
    bot: Single<&mut LinearVelocity, (With<Bot>, Without<Ball>)>,
    ball: Single<&mut LinearVelocity, With<Ball>>,
) {
    // Follower bot
    let ball_vel = ball.into_inner();
    let mut bot_vel = bot.into_inner();
    bot_vel.y = ((1.0 + rand::random::<f32>()) * ball_vel.y).clamp(-BOT_MAX_SPEED, BOT_MAX_SPEED);
}
