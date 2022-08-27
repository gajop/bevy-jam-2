use bevy::{prelude::*, render::view::RenderLayers};

use crate::game_mechanics::{GameColor, Goal, GridPos, Player, Trap, GRID_SIZE_Y};

pub struct ObjectRenderingPlugin;

impl Plugin for ObjectRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_object)
            .add_system(spawn_trap_object)
            .add_system(spawn_goal_object)
            .add_system(update_visibility)
            .add_system(update_transform_from_grid);
    }
}

fn spawn_player_object(
    q_added_player: Query<(Entity, &GridPos), Added<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    });

    for entity_pos in q_added_player.iter() {
        let entity = entity_pos.0;
        let pos = entity_pos.1;

        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            material: material_handle.clone(),
            transform: Transform::from_translation(grid_to_translation(*pos)),

            ..default()
        });
    }
}

fn spawn_trap_object(
    q_added_trap: Query<(Entity, &GridPos), Added<Trap>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    });

    for entity_pos in q_added_trap.iter() {
        let entity = entity_pos.0;
        let pos = entity_pos.1;

        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: material_handle.clone(),
            transform: Transform::from_translation(grid_to_translation(*pos)),

            ..default()
        });
    }
}

fn spawn_goal_object(
    q_added_goal: Query<(Entity, &GridPos), Added<Goal>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    });

    for entity_pos in q_added_goal.iter() {
        let entity = entity_pos.0;
        let pos = entity_pos.1;

        commands.entity(entity).insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                radius: 0.5,
                ring_radius: 0.1,
                ..default()
            })),
            material: material_handle.clone(),
            transform: Transform::from_translation(grid_to_translation(*pos)),

            ..default()
        });
    }
}

fn update_visibility(
    q_game_colors: Query<(Entity, &GameColor), Changed<GameColor>>,
    mut commands: Commands,
) {
    for entity_color in q_game_colors.iter() {
        let entity = entity_color.0;
        let color = entity_color.1;

        commands.entity(entity).remove::<RenderLayers>();
        let layers = layers_from_game_color(*color);
        dbg!("layers = {:?}", &layers);
        commands.entity(entity).insert(layers);
    }
}

fn layers_from_game_color(game_color: GameColor) -> RenderLayers {
    let layers: Vec<u8> = match game_color {
        GameColor::Red => vec![1],
        GameColor::Green => vec![2],
        GameColor::Blue => vec![3],
        GameColor::Yellow => vec![1, 2],
        GameColor::Cyan => vec![2, 3],
        GameColor::Pink => vec![1, 3],
        GameColor::White => vec![1, 2, 3],
    };
    let layers = layers.as_slice();

    RenderLayers::from_layers(layers)
}

fn update_transform_from_grid(
    mut q_transform_pos: Query<(&mut Transform, &GridPos), Or<(Added<GridPos>, Changed<GridPos>)>>,
) {
    for transform_pos in q_transform_pos.iter_mut() {
        let mut transform = transform_pos.0;
        let pos = transform_pos.1;

        transform.translation = grid_to_translation(*pos);
    }
}

fn grid_to_translation(pos: GridPos) -> Vec3 {
    Vec3::new(pos.x as f32, 1.0, (GRID_SIZE_Y - pos.y - 1) as f32)
}
