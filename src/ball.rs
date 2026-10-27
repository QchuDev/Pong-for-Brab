use crate::kinematics::*;
use bevy::prelude::*;

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball);
    }
}

#[derive(Component)]
struct Ball;

pub fn spawn_ball(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("pong-spritesheet.png");
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
        Transform::from_translation(Vec3::ZERO),
        Velocity { x: 20.0, y: 0.0 },
    ));
}
