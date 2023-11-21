use bevy::prelude::*;
use rand::{thread_rng, Rng};

use self::JudgmentLevel::{Bad, Good, None, Perfect};

// キーを変更
// TODO: 構造体で扱うかも
pub static mut TIMING_KEY_A: KeyCode = KeyCode::D;
pub static mut TIMING_KEY_B: KeyCode = KeyCode::F;
pub static mut TIMING_KEY_C: KeyCode = KeyCode::J;
pub static mut TIMING_KEY_D: KeyCode = KeyCode::K;

const BLOCK_SIZE: Vec2 = Vec2::new(70., 20.);
const BLOCK_NORMAL_COLOR: Color = Color::GRAY;
const BLOCK_PRESSED_COLOR: Color = Color::RED;

const TIMING_BLOCK_POSITION_Y: f32 = -220.0;

const TIMING_BLOCK_A: Vec2 = Vec2::new(-BLOCK_SIZE.x * 2.0, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_B: Vec2 = Vec2::new(-BLOCK_SIZE.x / 1.5, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_C: Vec2 = Vec2::new(BLOCK_SIZE.x / 1.5, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_D: Vec2 = Vec2::new(BLOCK_SIZE.x * 2.0, TIMING_BLOCK_POSITION_Y);

const MUSIC_BLOCK_START: f32 = 300.0;
const _MUSIC_BLOCK_SPEED: f32 = 550.0;
const MUSIC_BLOCK_DESPAWN_POINT: f32 = -400.0;
const MUSIC_BLOCK_COLOR: Color = Color::YELLOW;

type JudLevel = (f32, f32);
const JUDGMENT_LEVEL_PERF: JudLevel = (-210., -230.); // 判定の範囲
const JUDGMENT_LEVEL_GOOD: JudLevel = (-180., -250.);
const JUDGMENT_LEVEL_BAD: JudLevel = (-160., -270.);

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
        .init_resource::<Scoreboard>()
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_event::<KeyPressEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .add_systems(
            Update,
            (
                keyboard_input,
                music_block_move,
                despawn_music_block,
                block_judgement,
            ),
        )
        .add_systems(FixedUpdate, create_music_block)
        .run();
}

#[derive(Component)]
struct MusicBlock(BlockLocation);

#[derive(Component)]
struct TimingBlock(BlockLocation);

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

#[derive(Clone, Copy, PartialEq)]
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
                    scale: BLOCK_SIZE.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: MUSIC_BLOCK_COLOR,
                    ..default()
                },
                ..default()
            },
            block: MusicBlock(location),
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
        TimingBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.timing_position().extend(0.0),
                    scale: BLOCK_SIZE.extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: BLOCK_NORMAL_COLOR,
                    ..default()
                },
                ..default()
            },
            block: TimingBlock(location),
        }
    }
}

#[derive(Event)]
struct KeyPressEvent {
    block_location: BlockLocation,
}

#[warn(warnings)]
#[derive(Resource, Default, Debug)]
struct Scoreboard {
    combo: usize,
    perfect: usize,
    good: usize,
    bad: usize,
}

enum _BlockKind {}

#[derive(Debug, PartialEq)]
enum JudgmentLevel {
    _Wonderful,
    Perfect,
    Good,
    Bad,
    None,
}

impl JudgmentLevel {
    fn check(block_position: f32) -> JudgmentLevel {
        match block_position {
            y if tuple_if(JUDGMENT_LEVEL_PERF, y) => Perfect,
            y if tuple_if(JUDGMENT_LEVEL_GOOD, y) => Good,
            y if tuple_if(JUDGMENT_LEVEL_BAD, y) => Bad,
            _ => None,
        }
    }
}
fn tuple_if(tuple: JudLevel, target: f32) -> bool {
    tuple.1 <= target && target <= tuple.0
}

impl Scoreboard {
    fn combo(&mut self) {
        self.combo += 1
    }
    fn combo_reset(&mut self) {
        self.combo = 0
    }
    fn perfect(&mut self) {
        self.perfect += 1;
        self.combo();
    }
    fn good(&mut self) {
        self.good += 1;
        self.combo_reset();
    }
    fn bad(&mut self) {
        self.bad += 1;
        self.combo_reset();
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
    mut key_event: EventWriter<KeyPressEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut sprite, key) in &mut timing_block_query {
        if keyboard_input.pressed(key.0.get_key()) {
            sprite.color = BLOCK_PRESSED_COLOR;

            key_event.send(KeyPressEvent {
                block_location: key.0,
            });
        } else {
            sprite.color = BLOCK_NORMAL_COLOR
        }
    }
}

fn block_judgement(
    mut commands: Commands,
    music_block_query: Query<(Entity, &Transform, &MusicBlock)>,
    mut key_press_event: EventReader<KeyPressEvent>,
    mut score_board: ResMut<Scoreboard>,
) {
    for event in key_press_event.read() {
        for (entity, transform, music_block) in &music_block_query {
            if music_block.0 == event.block_location {
                let music_y = transform.translation.y;
                let level = JudgmentLevel::check(music_y);

                if level != None {
                    match level {
                        Perfect => score_board.perfect(),
                        Good => score_board.good(),
                        Bad => score_board.bad(),
                        _ => (),
                    }
                    println!("{:#?}", score_board);
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

fn music_block_move(mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.y += velocity.0
    }
}

fn create_music_block(mut commands: Commands) {
    commands.spawn(MusicBundle::rand_block());
}

fn despawn_music_block(
    mut commands: Commands,
    music_block_query: Query<(Entity, &Transform), With<MusicBlock>>,
    mut score_board: ResMut<Scoreboard>,
) {
    for (entity, transform) in &music_block_query {
        if transform.translation.y < MUSIC_BLOCK_DESPAWN_POINT {
            commands.entity(entity).despawn();
            score_board.combo_reset();
        }
    }
}
