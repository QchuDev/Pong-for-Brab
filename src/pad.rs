use crate::bot::Bot;
use crate::player::Player;
use crate::points::Team;
use avian2d::collision::collider::Collider;
use avian2d::dynamics::rigid_body::{Dominance, LockedAxes, Restitution, RigidBody};
use bevy::prelude::*;

pub const PAD_DISPLACEMENT: f32 = 250.0;

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
    let texture = asset_server.load("sprites/pong-spritesheet.png");
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
        Bot,
        Pad,
        Team::RIGHT,
        RigidBody::Dynamic,
        Dominance(1),
        LockedAxes::ROTATION_LOCKED.lock_translation_x(),
        Restitution::default(),
        Transform::from_xyz(PAD_DISPLACEMENT, 0.0, 0.0),
        Collider::rectangle(6.0, 32.0),
    ));

    // Spawn Players pad
    commands.spawn((
        pad_sprite,
        Player,
        Pad,
        Team::LEFT,
        RigidBody::Dynamic,
        Dominance(1),
        LockedAxes::ROTATION_LOCKED.lock_translation_x(),
        Restitution::default(),
        Transform::from_xyz(-PAD_DISPLACEMENT, 0.0, 0.0),
        Collider::rectangle(6.0, 32.0),
    ));
}
