use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::{TypeUuid, TypePath},
    render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::{PrimaryWindow, WindowResized},
};
use ctrl_macros::ok_or_return;

use crate::game_mechanics::{GRID_SIZE_X, GRID_SIZE_Y};

pub struct CameraRendering;

#[derive(Component)]
struct CameraStuff;

impl Plugin for CameraRendering {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<PostProcessingMaterialRed>::default())
            .add_plugins(Material2dPlugin::<PostProcessingMaterialGreen>::default())
            .add_plugins(Material2dPlugin::<PostProcessingMaterialBlue>::default())
            .add_systems(Startup, setup_main_camera)
            .add_systems(Startup, setup)
            .add_systems(Startup, setup_cameras)
            .add_systems(Startup, spawn_first_level)
            // .add_systems(Update, set_camera_viewports)
            .add_systems(Update, recreate_on_resize);
        // .add_systems(Update, resize_camera_sprites);
    }
}

// fn setup_cameras(
//     mut images: ResMut<Assets<Image>>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut windows: ResMut<Windows>,
// ) {
// }

fn setup_cameras(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut post_processing_materials_red: ResMut<Assets<PostProcessingMaterialRed>>,
    mut post_processing_materials_green: ResMut<Assets<PostProcessingMaterialGreen>>,
    mut post_processing_materials_blue: ResMut<Assets<PostProcessingMaterialBlue>>,
    mut meshes: ResMut<Assets<Mesh>>,
    q_window: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    let window = ok_or_return!(q_window.get_single()).1;
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    for i in 0..3 {
        let mut image = Image {
            texture_descriptor: TextureDescriptor {
                label: None,
                size,
                dimension: TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,

                view_formats: &[],
            },
            ..default()
        };

        // fill image.data with zeroes
        image.resize(size);

        let image_handle = images.add(image);

        let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
            size.width as f32 / 3.0,
            size.height as f32,
        ))));

        // This material has the texture that has been rendered.
        let transform =
            Transform::from_xyz((size.width as f32 / 3.0 + 5.0) * (i as f32 - 1.0), 0., 0.);

        match i {
            0 => {
                let material_handle =
                    post_processing_materials_red.add(PostProcessingMaterialRed {
                        source_image: image_handle.clone(),
                    });

                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: quad_handle.into(),
                        material: material_handle,
                        transform,
                        ..default()
                    })
                    .insert(CameraStuff);
            }
            1 => {
                let material_handle =
                    post_processing_materials_green.add(PostProcessingMaterialGreen {
                        source_image: image_handle.clone(),
                    });

                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: quad_handle.into(),
                        material: material_handle,
                        transform,
                        ..default()
                    })
                    .insert(CameraStuff);
            }
            2 => {
                let material_handle =
                    post_processing_materials_blue.add(PostProcessingMaterialBlue {
                        source_image: image_handle.clone(),
                    });

                commands
                    .spawn(MaterialMesh2dBundle {
                        mesh: quad_handle.into(),
                        material: material_handle,
                        transform,
                        ..default()
                    })
                    .insert(CameraStuff);
            }
            _ => {
                continue;
            }
        };

        let mut cmd = commands.spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Default,
                ..default()
            },

            // camera_2d: Camera2d
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                order: -1,
                viewport: Some(Viewport {
                    physical_position: UVec2::new((i as u32) * window.physical_width() / 3, 0),
                    physical_size: UVec2::new(
                        window.physical_width() / 3,
                        window.physical_height(),
                    ),
                    ..default()
                }),
                ..default()
            },
            transform: Transform::from_xyz(3.0, 35.0, 12.0)
                .looking_at(Vec3::new(3.0, 10.0, 5.0), Vec3::Y),

            ..default()
        });
        cmd.insert(RenderLayers::layer(i + 1))
            .insert(CameraStuff)
            .insert(UiCameraConfig { show_ui: false });

        match i {
            0 => {
                cmd.insert(RedCamera);
                cmd.insert(Name::new("Red Camera"));
            }
            1 => {
                cmd.insert(GreenCamera);
                cmd.insert(Name::new("Green Camera"));
            }
            2 => {
                cmd.insert(BlueCamera);
                cmd.insert(Name::new("Blue Camera"));
            }
            _ => continue,
        };
    }
}

fn setup_main_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(Camera2dBundle {
    //     camera: todo!(),
    //     camera_render_graph: todo!(),
    //     projection: todo!(),
    //     visible_entities: todo!(),
    //     frustum: todo!(),
    //     transform: todo!(),
    //     global_transform: todo!(),
    //     camera_2d: todo!(),
    // });
}

fn setup(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn spawn_first_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.8, 0.8),
        ..default()
    });

    let element_size = 0.95;
    let padding = 0.05;

    for i in 0..GRID_SIZE_X {
        for j in 0..GRID_SIZE_Y {
            let x = i as f32;
            let y = j as f32;
            let x = x * (element_size + padding);
            let y = y * (element_size + padding);
            commands
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: element_size })),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(x, 0.0, y),
                    ..default()
                })
                .insert(RenderLayers::from_layers(&[1, 2, 3]))
                .insert(Name::new("Plane"));
        }
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, TypePath)]
#[uuid = "bc1812d4-ba8e-4cc8-87c1-84ef07a7cb7b"]
struct PostProcessingMaterialRed {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterialRed {
    fn fragment_shader() -> ShaderRef {
        "shaders/red_view.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, TypePath)]
#[uuid = "c806abff-a13f-4173-be15-9810b7626888"]
struct PostProcessingMaterialGreen {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterialGreen {
    fn fragment_shader() -> ShaderRef {
        "shaders/green_view.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone, TypePath)]
#[uuid = "acd3ba48-bbe7-45e6-a5c8-aec911b6dad6"]
struct PostProcessingMaterialBlue {
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterialBlue {
    fn fragment_shader() -> ShaderRef {
        "shaders/blue_view.wgsl".into()
    }
}

#[derive(Component)]
struct RedCamera;

#[derive(Component)]
struct GreenCamera;

#[derive(Component)]
struct BlueCamera;

// fn set_camera_viewports(
//     windows: Res<Windows>,
//     mut resize_events: EventReader<WindowResized>,
//     mut red_camera: Query<
//         &mut Camera,
//         (With<RedCamera>, Without<BlueCamera>, Without<GreenCamera>),
//     >,
//     mut green_camera: Query<
//         &mut Camera,
//         (With<GreenCamera>, Without<RedCamera>, Without<BlueCamera>),
//     >,
//     mut blue_camera: Query<
//         &mut Camera,
//         (With<BlueCamera>, Without<RedCamera>, Without<GreenCamera>),
//     >,
// ) {
//     for resize_event in resize_events.iter() {
//         if resize_event.id == WindowId::primary() {
//             let window = windows.primary();
//             let mut red_camera = red_camera.single_mut();
//             red_camera.viewport = Some(Viewport {
//                 physical_position: UVec2::new(0, 0),
//                 physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
//                 ..default()
//             });

//             let mut green_camera = green_camera.single_mut();
//             green_camera.viewport = Some(Viewport {
//                 physical_position: UVec2::new(window.physical_width() / 3, 0),
//                 physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
//                 ..default()
//             });

//             let mut blue_camera = blue_camera.single_mut();
//             blue_camera.viewport = Some(Viewport {
//                 physical_position: UVec2::new(2 * window.physical_width() / 3, 0),
//                 physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
//                 ..default()
//             });
//         }
//     }
// }

// fn resize_camera_sprites(
//     windows: Res<Windows>,
//     mut resize_events: EventReader<WindowResized>,
//     mut q_camera_sprite_transform: Query<&mut Transform, With<CameraSprite>>,
// ) {
//     for resize_event in resize_events.iter() {
//         if resize_event.id != WindowId::primary() {
//             continue;
//         }
//         let window = windows.primary();
//         let size = Extent3d {
//             width: window.physical_width(),
//             height: window.physical_height(),
//             ..default()
//         };

//         for (i, mut transform) in q_camera_sprite_transform.iter_mut().enumerate() {
//             transform.translation.x = (size.width as f32 / 3.0 + 5.0) * (i as f32 - 1.0);
//         }
//     }
// }

fn recreate_on_resize(
    images: ResMut<Assets<Image>>,
    mut commands: Commands,
    post_processing_materials_red: ResMut<Assets<PostProcessingMaterialRed>>,
    post_processing_materials_green: ResMut<Assets<PostProcessingMaterialGreen>>,
    post_processing_materials_blue: ResMut<Assets<PostProcessingMaterialBlue>>,
    meshes: ResMut<Assets<Mesh>>,
    q_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut resize_events: EventReader<WindowResized>,
    q_camera_stuff: Query<Entity, With<CameraStuff>>,
) {
    let window_entity = ok_or_return!(q_window.get_single()).0;
    let mut has_resize = false;
    for resize_event in resize_events.iter() {
        // TODO: Check if I should use let window = ok_or_return!(q_window.get_single());
        if resize_event.window != window_entity {
            continue;
        }
        has_resize = true;
    }

    // CameraStuff

    // return;

    if !has_resize {
        return;
    }

    for entity in q_camera_stuff.iter() {
        commands.entity(entity).despawn();
    }

    setup_cameras(
        images,
        commands,
        post_processing_materials_red,
        post_processing_materials_green,
        post_processing_materials_blue,
        meshes,
        q_window,
    );
}
