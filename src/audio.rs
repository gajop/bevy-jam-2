use bevy::prelude::*;

use crate::game_mechanics::{HitTrapEvent, PlayerMovedEvent, ReachedGoalEvent};

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_on_goal_reached)
            .add_system(play_on_hit_trap)
            .add_system(play_on_change_pos);
    }
}

fn play_on_goal_reached(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut event: EventReader<ReachedGoalEvent>,
) {
    for _ in event.iter() {
        let music = asset_server.load("sounds/level-reached.ogg");
        audio.play(music);
    }
}

fn play_on_hit_trap(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut event: EventReader<HitTrapEvent>,
) {
    for _ in event.iter() {
        let music = asset_server.load("sounds/hit-trap.ogg");
        audio.play(music);
    }
}

fn play_on_change_pos(
    mut event: EventReader<PlayerMovedEvent>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let music = asset_server.load("sounds/move.ogg");
    for _ in event.iter() {
        audio.play(music.clone());
    }
}
