use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use ctrl_macros::ok_or_return;
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

impl Plugin for GameMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system)
            .add_system(reach_goal)
            .add_system(hit_trap)
            .add_event::<HitTrapEvent>()
            .add_event::<ReachedGoalEvent>();

        const DEBUG: bool = true;
        if DEBUG {
            app.register_inspectable::<GridPos>()
                .register_inspectable::<GameColor>();
        }
    }
}
fn movement_system(mut q_player_pos: Query<&mut GridPos, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut player_pos = ok_or_return!(q_player_pos.get_single_mut());

    if keys.just_pressed(KeyCode::W) {
        player_pos.y = (player_pos.y + 1).min(GRID_SIZE_Y - 1);
    } else if keys.just_pressed(KeyCode::A) {
        player_pos.x = (player_pos.x - 1).max(0);
    } else if keys.just_pressed(KeyCode::S) {
        player_pos.y = (player_pos.y - 1).max(0);
    } else if keys.just_pressed(KeyCode::D) {
        player_pos.x = (player_pos.x + 1).min(GRID_SIZE_X - 1);
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

pub struct ReachedGoalEvent;
pub struct HitTrapEvent;
