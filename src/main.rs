use std::time::Duration;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

use self::JudgmentLevel::{Bad, Good, Miss, None, Perfect, Wonderful};

// キーを変更
// TODO: 構造体で扱うかも
pub static mut TIMING_KEY_A: KeyCode = KeyCode::D;
pub static mut TIMING_KEY_B: KeyCode = KeyCode::F;
pub static mut TIMING_KEY_C: KeyCode = KeyCode::J;
pub static mut TIMING_KEY_D: KeyCode = KeyCode::K;

// 全てのブロックのサイズ
const BLOCK_SIZE: Vec2 = Vec2::new(70., 20.);

const TIMING_BLOCK_POSITION_Y: f32 = -220.0;
const TIMING_BLOCK_A: Vec2 = Vec2::new(-BLOCK_SIZE.x * 2.0, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_B: Vec2 = Vec2::new(-BLOCK_SIZE.x / 1.5, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_C: Vec2 = Vec2::new(BLOCK_SIZE.x / 1.5, TIMING_BLOCK_POSITION_Y);
const TIMING_BLOCK_D: Vec2 = Vec2::new(BLOCK_SIZE.x * 2.0, TIMING_BLOCK_POSITION_Y);

const _MUSIC_BLOCK_SPEED: f32 = 550.0;
// music blockの出現位置
const MUSIC_BLOCK_START: f32 = 300.0;
// music blcokの消滅位置
const MUSIC_BLOCK_DESPAWN_POINT: f32 = -400.0;

type JudLevel = (f32, f32);
const JUDGMENT_LEVEL_PERF: JudLevel = (-210., -230.); // 判定の範囲
const JUDGMENT_LEVEL_GOOD: JudLevel = (-180., -250.);
const JUDGMENT_LEVEL_BAD: JudLevel = (-160., -270.);

const SCOREBOARD_TEXT_SIZE: f32 = 30.0;
const SCOREBOARD_PADDING: Val = Val::Px(4.0);
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(3.0);

const MUSIC_BLOCK_COLOR: Color = Color::YELLOW;
const BLOCK_NORMAL_COLOR: Color = Color::GRAY;
const BLOCK_PRESSED_COLOR: Color = Color::RED;
const SCOREBOARD_TEXT_COLOR: Color = Color::WHITE;

mod music_block_scale {
    use bevy::math::Vec2;

    use crate::BLOCK_SIZE;

    pub const NORMAL: Vec2 = BLOCK_SIZE;
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Music Game".into(),
                resolution: (600., 820.).into(),
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..default()
                },
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Scoreboard {
            combo: 0,
            wonderful: 0,
            perfect: 0,
            good: 0,
            bad: 0,
            miss: 0,
        })
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
                update_scoreboard,
            ),
        )
        .add_systems(FixedUpdate, create_music_block)
        .add_systems(Update, fixed_time_change)
        .run();
}

#[derive(Component)]
struct MusicBlock {
    location: BlockLocation,
    kind: MusicBlockKind,
}

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
#[derive(Event)]
struct KeyPressEvent {
    block_location: BlockLocation,
}

#[warn(warnings)]
#[derive(Resource)]
struct Scoreboard {
    combo: usize,
    wonderful: usize,
    perfect: usize,
    good: usize,
    bad: usize,
    miss: usize,
}

// TODO: music blckの種類の追加
enum MusicBlockKind {
    Normal,
}

#[derive(Component, PartialEq)]
enum JudgmentLevel {
    Wonderful,
    Perfect,
    Good,
    Bad,
    Miss,
    None,
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

impl MusicBlockKind {
    fn scale(&self) -> Vec2 {
        use self::music_block_scale::*;

        match self {
            MusicBlockKind::Normal => NORMAL,
        }
    }
}
impl MusicBundle {
    fn new(location: BlockLocation, kind: MusicBlockKind) -> Self {
        MusicBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.music_position().extend(0.0),
                    scale: kind.scale().extend(0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: MUSIC_BLOCK_COLOR,
                    ..default()
                },
                ..default()
            },
            block: MusicBlock { location, kind },
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
        let kind = MusicBlockKind::Normal;

        MusicBundle::new(location, kind)
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

impl JudgmentLevel {
    fn check(block_position: f32) -> JudgmentLevel {
        match block_position {
            // TIMING_BLOCK_POSITION_Y => Wonderful, // これだと↓
            // warning: floating-point types cannot be used in patterns
            // warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
            // note: for more information, see issue #41620 <https://github.com/rust-lang/rust/issues/41620>
            // note: `#[warn(illegal_floating_point_literal_pattern)]` on by default
            // 合っているかわからないけど一旦これで
            y if TIMING_BLOCK_POSITION_Y == y => Wonderful,
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
    fn wonderful(&mut self) {
        self.wonderful += 1;
        self.combo();
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
    fn miss(&mut self) {
        self.miss += 1;
        self.combo_reset();
    }
}

fn setup(mut commands: Commands) {
    let text_style = TextStyle {
        font_size: SCOREBOARD_TEXT_SIZE,
        color: SCOREBOARD_TEXT_COLOR,
        ..default()
    };
    let with_text_style = Style {
        top: SCOREBOARD_TEXT_PADDING,
        left: SCOREBOARD_TEXT_PADDING,
        ..default()
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn(TimingBundle::new(BlockLocation::A));
    commands.spawn(TimingBundle::new(BlockLocation::B));
    commands.spawn(TimingBundle::new(BlockLocation::C));
    commands.spawn(TimingBundle::new(BlockLocation::D));

    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                top: SCOREBOARD_PADDING,
                left: SCOREBOARD_PADDING,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Wonderful: ", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ])
                .with_style(with_text_style.clone()),
                Wonderful,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Perfect: ", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ])
                .with_style(with_text_style.clone()),
                Perfect,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Good: ", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ])
                .with_style(with_text_style.clone()),
                Good,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Bad: ", text_style.clone()),
                    TextSection::from_style(text_style.clone()),
                ])
                .with_style(with_text_style.clone()),
                Bad,
            ));
            parent.spawn((
                TextBundle::from_sections([
                    TextSection::new("Miss: ", text_style.clone()),
                    TextSection::from_style(text_style),
                ])
                .with_style(with_text_style),
                Miss,
            ));
        });
}

fn keyboard_input(
    mut timing_block_query: Query<(&mut Sprite, &TimingBlock)>,
    mut key_event: EventWriter<KeyPressEvent>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    for (mut sprite, key) in &mut timing_block_query {
        if keyboard_input.just_pressed(key.0.get_key()) {
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
    // KeyPressEventを
    'event: for event in key_press_event.read() {
        for (entity, transform, music_block) in &music_block_query {
            if music_block.location == event.block_location {
                let music_y = transform.translation.y;
                let level = JudgmentLevel::check(music_y);

                if level != None {
                    match level {
                        Wonderful => score_board.wonderful(),
                        Perfect => score_board.perfect(),
                        Good => score_board.good(),
                        Bad => score_board.bad(),
                        _ => (),
                    }

                    commands.entity(entity).despawn();

                    break 'event;
                }
            }
        }
    }
}

fn update_scoreboard(
    score_board: Res<Scoreboard>,
    mut text_query: Query<(&mut Text, &JudgmentLevel)>,
) {
    for (mut text, level) in &mut text_query {
        text.sections[1].value = match level {
            Wonderful => score_board.wonderful.to_string(),
            Perfect => score_board.perfect.to_string(),
            Good => score_board.good.to_string(),
            Bad => score_board.bad.to_string(),
            Miss => score_board.miss.to_string(),
            _ => "".to_string(),
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
            score_board.miss();
        }
    }
}

fn fixed_time_change(mut time: ResMut<Time<Fixed>>) {
    let rand = thread_rng().gen_range(0.3..=2.0);
    time.set_timestep(Duration::from_secs_f32(rand));
}
