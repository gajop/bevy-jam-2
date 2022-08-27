use bevy::prelude::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use ctrl_macros::ok_or_return;

pub const GRID_SIZE_X: i32 = 8;
pub const GRID_SIZE_Y: i32 = 12;

pub struct GameMechanicsPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Goal;

#[derive(Component)]
pub struct Trap;

#[derive(Component, Inspectable, Copy, Clone)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Inspectable, Copy, Clone)]
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
        app.add_startup_system(spawn_player)
            .add_system(movement_system)
            .add_system(reach_goal)
            .add_system(hit_trap);

        const DEBUG: bool = true;
        if DEBUG {
            app.register_inspectable::<GridPos>()
                .register_inspectable::<GameColor>();
        }
    }
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn()
        .insert(Player)
        .insert(GridPos { x: 5, y: 5 })
        .insert(GameColor::White)
        .insert(Name::new("Player"));
}

fn movement_system(mut q_player_pos: Query<&mut GridPos, With<Player>>, keys: Res<Input<KeyCode>>) {
    let mut player_pos = ok_or_return!(q_player_pos.get_single_mut());

    if keys.just_pressed(KeyCode::W) {
        player_pos.y += 1;
    } else if keys.just_pressed(KeyCode::A) {
        player_pos.x -= 1;
    } else if keys.just_pressed(KeyCode::S) {
        player_pos.y -= 1;
    } else if keys.just_pressed(KeyCode::D) {
        player_pos.x += 1;
    }

    player_pos.x = player_pos.x.clamp(0, GRID_SIZE_X - 1);
    player_pos.y = player_pos.y.clamp(0, GRID_SIZE_Y - 1);
}

fn reach_goal() {}

fn hit_trap() {}
