use bevy::prelude::*;

use crate::game_mechanics::{HitTrapEvent, PlayerMovedEvent, ReachedGoalEvent, TimerExpiredEvent};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, play_on_goal_reached)
            .add_systems(Update, play_on_hit_trap)
            .add_systems(Update, play_on_change_pos)
            .add_systems(Update, play_on_timer_elapse);
    }
}

fn play_on_goal_reached(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event: EventReader<ReachedGoalEvent>,
) {
    for _ in event.iter() {
        commands.spawn(
            AudioBundle {
                source: asset_server.load("sounds/level-reached.ogg"),
                ..default()
            }
        );
    }
}

fn play_on_hit_trap(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event: EventReader<HitTrapEvent>,
) {
    for _ in event.iter() {
        commands.spawn(
            AudioBundle {
                source: asset_server.load("sounds/hit-trap.ogg"),
                ..default()
            }
        );
    }
}

fn play_on_timer_elapse(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut event: EventReader<TimerExpiredEvent>,
) {
    for _ in event.iter() {
        commands.spawn(
            AudioBundle {
                source: asset_server.load("sounds/hit-trap.ogg"),
                ..default()
            }
        );
    }
}

fn play_on_change_pos(
    mut event: EventReader<PlayerMovedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in event.iter() {
        commands.spawn(
            AudioBundle {
                source: asset_server.load("sounds/move.ogg"),
                ..default()
            }
        );
    }
}
