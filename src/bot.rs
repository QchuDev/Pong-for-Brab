use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::ball::Ball;
const BOT_MAX_SPEED: f32 = 240.0;
const BOT_SPEED_MUTLIPLIER: f32 = 10.0;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, bot_following);
    }
}

#[derive(Component)]
pub struct Bot;

fn bot_following(
    bot: Single<(&Transform, &mut LinearVelocity), (With<Bot>, Without<Ball>)>,
    ball: Single<(&Transform, &mut LinearVelocity), With<Ball>>,
) {
    // Follower bot
    let (ball_transform, _ball_vel) = ball.into_inner();
    let (bot_transform, mut bot_vel) = bot.into_inner();

    let y_diff = ball_transform.translation.y - bot_transform.translation.y;
    bot_vel.y = (BOT_SPEED_MUTLIPLIER * y_diff).clamp(-BOT_MAX_SPEED, BOT_MAX_SPEED);
}
