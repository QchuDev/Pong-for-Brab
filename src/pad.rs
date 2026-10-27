use crate::kinematics::Velocity;
use crate::player::Player;
use bevy::prelude::*;

pub const PAD_DISPLACEMENT: f32 = 200.0;

pub struct PadPlugin;
impl Plugin for PadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pads);
    }
}

#[derive(Component)]
pub struct Pad;

pub fn spawn_pads(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("pong-spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(32),
        1,
        1,
        None,
        None,
    ));

    let pad_sprite = Sprite::from_atlas_image(texture, TextureAtlas { layout, index: 0 });

    // Spawn rivals pad
    commands.spawn((
        pad_sprite.clone(),
        Pad,
        Transform::from_translation(Vec3::new(PAD_DISPLACEMENT, 0.0, 0.0)),
        Velocity { x: 0.0, y: 0.0 },
    ));

    // Spawn Players pad
    commands.spawn((
        pad_sprite,
        Player,
        Pad,
        Transform::from_translation(Vec3::new(-PAD_DISPLACEMENT, 0.0, 0.0)),
        Velocity { x: 0.0, y: 0.0 },
    ));
}
