use bevy::{prelude::*, render::view::RenderLayers};

use crate::game_mechanics::{GameColor, Goal, GridPos, Player, Trap, GRID_SIZE_Y};

pub struct ObjectRenderingPlugin;

impl Plugin for ObjectRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player_object)
            .add_system(spawn_trap_object)
            .add_system(spawn_goal_object)
            .add_system(update_material_color)
            .add_system(update_visibility)
            .add_system(update_transform_from_grid);
    }
}

fn spawn_player_object(
    q_added_player: Query<(Entity, &GridPos, &GameColor), Added<Player>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, &pos, &color) in q_added_player.iter() {
        spawn_world_object(
            entity,
            color,
            pos,
            &mut commands,
            meshes.add(Mesh::from(shape::UVSphere {
                radius: 0.5,
                ..default()
            })),
            &mut materials,
        );
    }
}

fn spawn_trap_object(
    q_added_trap: Query<(Entity, &GridPos, &GameColor), Added<Trap>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, &pos, &color) in q_added_trap.iter() {
        spawn_world_object(
            entity,
            color,
            pos,
            &mut commands,
            meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            &mut materials,
        );
    }
}

fn spawn_goal_object(
    q_added_goal: Query<(Entity, &GridPos, &GameColor), Added<Goal>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, &pos, &color) in q_added_goal.iter() {
        spawn_world_object(
            entity,
            color,
            pos,
            &mut commands,
            meshes.add(Mesh::from(shape::Torus {
                radius: 0.5,
                ring_radius: 0.1,
                ..default()
            })),
            &mut materials,
        );
    }
}

fn spawn_world_object(
    entity: Entity,
    color: GameColor,
    pos: GridPos,
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: get_bevy_color(color),
        ..default()
    });

    commands.entity(entity).insert_bundle(PbrBundle {
        mesh,
        material: material_handle,
        transform: Transform::from_translation(grid_to_translation(pos)),

        ..default()
    });
}

fn update_visibility(
    q_game_colors: Query<(Entity, &GameColor), Changed<GameColor>>,
    mut commands: Commands,
) {
    for entity_color in q_game_colors.iter() {
        let entity = entity_color.0;
        let color = *entity_color.1;

        commands.entity(entity).remove::<RenderLayers>();
        let layers = layers_from_game_color(color);
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

fn update_material_color(
    mut q_game_materials: Query<(&GameColor, &mut Handle<StandardMaterial>), Changed<GameColor>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (&color, material_handle) in &mut q_game_materials.iter_mut() {
        let mut color_mat = materials.get_mut(&material_handle).unwrap();
        color_mat.base_color = get_bevy_color(color);
    }
}

fn get_bevy_color(game_color: GameColor) -> Color {
    match game_color {
        GameColor::Red => Color::rgb(1.0, 0.0, 0.0),
        GameColor::Green => Color::rgb(0.0, 1.0, 0.0),
        GameColor::Blue => Color::rgb(0.0, 0.0, 1.0),
        GameColor::Yellow => Color::rgb(1.0, 1.0, 0.0),
        GameColor::Cyan => Color::rgb(0.0, 1.0, 1.0),
        GameColor::Pink => Color::rgb(1.0, 0.0, 1.0),
        GameColor::White => Color::rgb(1.0, 1.0, 1.0),
    }
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
