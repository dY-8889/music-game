use bevy::prelude::*;
use rand::{thread_rng, Rng};

pub static mut TIMING_KEY_A: KeyCode = KeyCode::D;
pub static mut TIMING_KEY_B: KeyCode = KeyCode::F;
pub static mut TIMING_KEY_C: KeyCode = KeyCode::J;
pub static mut TIMING_KEY_D: KeyCode = KeyCode::K;

const BLOCK_SIZE: Vec3 = Vec3::new(70., 20., 0.);
const TIMING_BLOCK_Y: f32 = -220.0;

const TIMING_BLOCK_A: Vec2 = Vec2::new(-BLOCK_SIZE.x * 2.0, TIMING_BLOCK_Y);
const TIMING_BLOCK_B: Vec2 = Vec2::new(-BLOCK_SIZE.x / 1.5, TIMING_BLOCK_Y);
const TIMING_BLOCK_C: Vec2 = Vec2::new(BLOCK_SIZE.x / 1.5, TIMING_BLOCK_Y);
const TIMING_BLOCK_D: Vec2 = Vec2::new(BLOCK_SIZE.x * 2.0, TIMING_BLOCK_Y);

const MUSIC_BLOCK_START: f32 = 300.0;
const MUSIC_BLOCK_SPEED: f32 = 550.0;
const MUSIC_BLOCK_DESPAWN_POINT: f32 = -400.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Music Game".into(),
                resolution: (600., 840.).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            (keyboard_input, music_block_move, despawn_music_block),
        )
        .add_systems(FixedUpdate, create_music_block)
        .run();
}

#[derive(Component)]
struct MusicBlock;

#[derive(Component)]
struct TimingBlock(KeyCode);

#[derive(Component, Deref, DerefMut)]
struct Velocity(f32);

#[derive(Bundle)]
struct MusicBundle {
    sprite_bundle: SpriteBundle,
    block: MusicBlock,
    velocity: Velocity,
}

#[derive(Bundle)]
struct TimingBundle {
    sprite_bundle: SpriteBundle,
    block: TimingBlock,
}

enum BlockLocation {
    A,
    B,
    C,
    D,
}

impl BlockLocation {
    fn timing_position(&self) -> Vec2 {
        match self {
            BlockLocation::A => TIMING_BLOCK_A,
            BlockLocation::B => TIMING_BLOCK_B,
            BlockLocation::C => TIMING_BLOCK_C,
            BlockLocation::D => TIMING_BLOCK_D,
        }
    }
    fn music_position(&self) -> Vec2 {
        match self {
            BlockLocation::A => Vec2::new(TIMING_BLOCK_A.x, MUSIC_BLOCK_START),
            BlockLocation::B => Vec2::new(TIMING_BLOCK_B.x, MUSIC_BLOCK_START),
            BlockLocation::C => Vec2::new(TIMING_BLOCK_C.x, MUSIC_BLOCK_START),
            BlockLocation::D => Vec2::new(TIMING_BLOCK_D.x, MUSIC_BLOCK_START),
        }
    }

    fn get_key(&self) -> KeyCode {
        unsafe {
            match self {
                BlockLocation::A => TIMING_KEY_A,
                BlockLocation::B => TIMING_KEY_B,
                BlockLocation::C => TIMING_KEY_C,
                BlockLocation::D => TIMING_KEY_D,
            }
        }
    }
}

impl MusicBundle {
    fn new(location: BlockLocation) -> Self {
        MusicBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.music_position().extend(0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                sprite: Sprite { ..default() },
                ..default()
            },
            block: MusicBlock,
            velocity: Velocity(-5.0),
        }
    }

    fn rand_block() -> MusicBundle {
        let location = match thread_rng().gen_range(0..=3) {
            0 => BlockLocation::A,
            1 => BlockLocation::B,
            2 => BlockLocation::C,
            3 => BlockLocation::D,
            _ => panic!(),
        };
        MusicBundle::new(location)
    }
}

impl TimingBundle {
    fn new(location: BlockLocation) -> Self {
        let key = location.get_key();

        TimingBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.timing_position().extend(0.0),
                    scale: BLOCK_SIZE,
                    ..default()
                },
                sprite: Sprite { ..default() },
                ..default()
            },
            block: TimingBlock(key),
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(TimingBundle::new(BlockLocation::A));
    commands.spawn(TimingBundle::new(BlockLocation::B));
    commands.spawn(TimingBundle::new(BlockLocation::C));
    commands.spawn(TimingBundle::new(BlockLocation::D));
}

fn keyboard_input(
    mut timing_block_query: Query<(&mut Sprite, &TimingBlock)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut sprite, key) in &mut timing_block_query {
        if keyboard_input.pressed(key.0) {
            sprite.color = Color::RED;
        } else {
            sprite.color = Color::WHITE;
        }
    }
}

fn music_block_move(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y += velocity.0;
    }
}

fn create_music_block(mut commands: Commands) {
    commands.spawn(MusicBundle::rand_block());
}

fn despawn_music_block(
    mut commands: Commands,
    music_block_query: Query<(Entity, &Transform), With<MusicBlock>>,
) {
    for (entity, transform) in &music_block_query {
        if transform.translation.y < MUSIC_BLOCK_DESPAWN_POINT {
            commands.entity(entity).despawn();
        }
    }
}
