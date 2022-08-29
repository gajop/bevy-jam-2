use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use ctrl_macros::some_or_return;
use serde::Deserialize;

use crate::game_mechanics::{
    GameColor, GameTimer, Goal, GridPos, HitTrapEvent, Player, ReachedGoalEvent, TimerExpiredEvent,
    Trap,
};

#[derive(Deserialize)]
struct LevelPlayer {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct LevelGoal {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
struct LevelTrap {
    x: i32,
    y: i32,
    color: GameColor,
}

#[derive(Deserialize)]
struct Level {
    player: LevelPlayer,
    goal: LevelGoal,
    traps: Vec<LevelTrap>,
}

#[derive(Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"] // <-- keep me unique
struct Levels {
    levels: Vec<Level>,
}

pub struct LevelInfo {
    pub desired_index: Option<i32>,
    pub index: Option<i32>,
    pub total_levels: usize,
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(JsonAssetPlugin::<Levels>::new(&["level.json"]))
            .add_startup_system(load_first_level)
            .add_startup_system(setup)
            .add_system(reload_level_on_death)
            .add_system(reload_level_on_timer_expired)
            .add_system(load_level_on_level_change)
            .add_system(go_to_next_level_on_goal)
            .insert_resource(LevelInfo {
                index: None,
                desired_index: None,
                total_levels: 0,
            });
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<Levels> = asset_server.load("levels.level.json");
    commands.insert_resource(handle);
}

fn load_first_level(mut level_info: ResMut<LevelInfo>) {
    level_info.desired_index = Some(0);
}

fn reload_level_on_death(
    mut ev_hit_trap: EventReader<HitTrapEvent>,
    mut level_info: ResMut<LevelInfo>,
) {
    for _ in ev_hit_trap.iter() {
        level_info.desired_index = level_info.index;
        level_info.index = None;
    }
}

fn reload_level_on_timer_expired(
    mut ev_hit_trap: EventReader<TimerExpiredEvent>,
    mut level_info: ResMut<LevelInfo>,
) {
    for _ in ev_hit_trap.iter() {
        level_info.desired_index = level_info.index;
        level_info.index = None;
    }
}

fn go_to_next_level_on_goal(
    mut ev: EventReader<ReachedGoalEvent>,
    mut level_info: ResMut<LevelInfo>,
) {
    for _ in ev.iter() {
        level_info.desired_index = Some(level_info.index.unwrap_or(0) + 1);
    }
}

fn load_level_on_level_change(
    mut commands: Commands,
    handle: Res<Handle<Levels>>,
    levels: Res<Assets<Levels>>,

    mut level_info: ResMut<LevelInfo>,

    q_existing_objects: Query<Entity, With<GridPos>>,

    timer: ResMut<GameTimer>,
) {
    if level_info.index == level_info.desired_index || level_info.desired_index.is_none() {
        return;
    }

    let levels = some_or_return!(levels.get(&handle));

    for entity in q_existing_objects.iter() {
        commands.entity(entity).despawn();
    }

    level_info.total_levels = levels.levels.len();

    level_info.index = level_info.desired_index;
    let index = level_info.index.unwrap_or(0) as usize;

    if index >= level_info.total_levels {
        return;
    }

    let level = &levels.levels[index];

    spawn_level(commands, level, timer);
}

fn spawn_level(mut commands: Commands, level: &Level, mut timer: ResMut<GameTimer>) {
    let player = &level.player;

    commands
        .spawn()
        .insert(Player)
        .insert(GridPos {
            x: player.x,
            y: player.y,
        })
        .insert(GameColor::White)
        .insert(Name::new("Player"));

    let goal = &level.goal;
    commands
        .spawn()
        .insert(Goal)
        .insert(GridPos {
            x: goal.x,
            y: goal.y,
        })
        .insert(GameColor::White)
        .insert(Name::new("Goal"));

    for trap in &level.traps {
        commands
            .spawn()
            .insert(Trap)
            .insert(GridPos {
                x: trap.x,
                y: trap.y,
            })
            .insert(trap.color)
            .insert(Name::new("Trap"));
    }

    timer.0 = Some(Timer::from_seconds(5.0, false));
}
