use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use ctrl_macros::{ok_or_return, some_or_return};
use serde::Deserialize;

pub const GRID_SIZE_X: i32 = 8;
pub const GRID_SIZE_Y: i32 = 12;

pub struct GameMechanicsPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct Trap;

#[derive(Component, Inspectable, Copy, Clone, PartialEq, Eq)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Inspectable, Copy, Clone, Deserialize)]
pub enum GameColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Pink,
    White,
}

// Events

pub struct ReachedGoalEvent;

pub struct HitTrapEvent;

pub struct PlayerMovedEvent;

pub struct TimerExpiredEvent;

pub struct GameTimer(pub Option<Timer>);

impl Plugin for GameMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system)
            .insert_resource(GameTimer(None))
            .add_system(reach_goal)
            .add_system(hit_trap)
            .add_system(timer_expired)
            .add_event::<HitTrapEvent>()
            .add_event::<ReachedGoalEvent>()
            .add_event::<PlayerMovedEvent>()
            .add_event::<TimerExpiredEvent>();

        const DEBUG: bool = true;
        if DEBUG {
            app.register_inspectable::<GridPos>()
                .register_inspectable::<GameColor>();
        }
    }
}
fn movement_system(
    mut q_player_pos: Query<&mut GridPos, With<Player>>,
    keys: Res<Input<KeyCode>>,
    mut ev_moved: EventWriter<PlayerMovedEvent>,
) {
    let mut pos = ok_or_return!(q_player_pos.get_single_mut());

    let mut moved = false;
    if keys.just_pressed(KeyCode::W) {
        if pos.y < GRID_SIZE_Y - 1 {
            pos.y += 1;
            moved = true;
        }
    } else if keys.just_pressed(KeyCode::A) {
        if pos.x > 0 {
            pos.x -= 1;
            moved = true;
        }
    } else if keys.just_pressed(KeyCode::S) {
        if pos.y > 0 {
            pos.y -= 1;
            moved = true;
        }
    } else if keys.just_pressed(KeyCode::D) {
        #[allow(clippy::collapsible_if)]
        if pos.x < GRID_SIZE_X - 1 {
            pos.x += 1;
            moved = true;
        }
    }

    if moved {
        ev_moved.send(PlayerMovedEvent);
    }
}

fn reach_goal(
    q_player_pos: Query<&GridPos, (With<Player>, Changed<GridPos>)>,
    q_goal_pos: Query<&GridPos, (With<Goal>, Without<Player>)>,
    mut ev_reached_goal: EventWriter<ReachedGoalEvent>,
) {
    let player_pos = ok_or_return!(q_player_pos.get_single());

    for goal in q_goal_pos.iter() {
        if player_pos == goal {
            ev_reached_goal.send(ReachedGoalEvent);
        }
    }
}

fn hit_trap(
    q_player_pos: Query<&GridPos, (With<Player>, Changed<GridPos>)>,
    q_trap_pos: Query<&GridPos, (With<Trap>, Without<Player>)>,
    mut ev_hit_trap: EventWriter<HitTrapEvent>,
) {
    let player_pos = ok_or_return!(q_player_pos.get_single());

    for trap in q_trap_pos.iter() {
        if player_pos == trap {
            ev_hit_trap.send(HitTrapEvent);
        }
    }
}

fn timer_expired(
    mut timer_res: ResMut<GameTimer>,
    time: Res<Time>,
    mut ev_timer_expired: EventWriter<TimerExpiredEvent>,
) {
    let timer = some_or_return!(&mut timer_res.0);
    timer.tick(time.delta());

    if timer.finished() {
        ev_timer_expired.send(TimerExpiredEvent);
        timer_res.0 = None;
    }
}
