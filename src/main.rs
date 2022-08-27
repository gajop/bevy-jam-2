use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{RenderTarget, Viewport},
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::{WindowId, WindowMode, WindowResized},
};

use bevy_inspector_egui::WorldInspectorPlugin;
use itertools::izip;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(WindowDescriptor {
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(Material2dPlugin::<PostProcessingMaterialRed>::default())
        .add_plugin(Material2dPlugin::<PostProcessingMaterialGreen>::default())
        .add_plugin(Material2dPlugin::<PostProcessingMaterialBlue>::default())
        .add_startup_system(setup)
        .add_startup_system(setup_cameras)
        .add_startup_system(spawn_first_level)
        .add_system(set_camera_viewports)
        .run();
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
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    // window.set_maximized(true);
    // let size = Extent3d {
    //     width: window.physical_width(),
    //     height: window.physical_height(),
    //     ..default()
    // };
    let size = Extent3d {
        width: 1980,
        height: 1080,
        ..default()
    };

    for i in 0..3 {
        // This is the texture that will be rendered to.
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
        let transform = Transform::from_xyz(
            ((size.width as f32 / 3.0 + 5.0) * (i as f32 - 1.0)) as f32,
            0.,
            0.,
        );

        match i {
            0 => {
                let material_handle =
                    post_processing_materials_red.add(PostProcessingMaterialRed {
                        source_image: image_handle.clone(),
                    });

                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: quad_handle.into(),
                    material: material_handle,
                    transform,
                    ..default()
                });
            }
            1 => {
                let material_handle =
                    post_processing_materials_green.add(PostProcessingMaterialGreen {
                        source_image: image_handle.clone(),
                    });

                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: quad_handle.into(),
                    material: material_handle,
                    transform,
                    ..default()
                });
            }
            2 => {
                let material_handle =
                    post_processing_materials_blue.add(PostProcessingMaterialBlue {
                        source_image: image_handle.clone(),
                    });

                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: quad_handle.into(),
                    material: material_handle,
                    transform,
                    ..default()
                });
            }
            _ => {
                continue;
            }
        };

        let mut cmd = commands.spawn_bundle(Camera3dBundle {
            camera_3d: Camera3d {
                // don't clear on the second camera because the first camera already cleared the window
                // clear_color: ClearColorConfig::None,
                ..default()
            },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                priority: -1,
                ..default()
            },
            transform: Transform::from_xyz(3.0, 35.0, 12.0)
                .looking_at(Vec3::new(3.0, 10.0, 5.0), Vec3::Y),
            ..default()
        });
        cmd.insert(RenderLayers::layer(i + 1));

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

    commands.spawn_bundle(Camera2dBundle::default());
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
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
        base_color: Color::rgb(1.0, 1.0, 1.0),
        ..default()
    });

    // commands
    //     .spawn_bundle(PbrBundle {
    //         mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
    //         material: material_handle,
    //         ..default()
    //     })
    //     .insert(RenderLayers::from_layers(&[1, 2, 3]))
    //     .insert(Name::new("Plane"));

    let grid_size_x = 8.0;
    let grid_size_y = 12.0;
    let element_size = 0.95;
    let padding = 0.05;

    for i in 0..grid_size_x as i32 {
        for j in 0..grid_size_y as i32 {
            let x = i as f32;
            let y = j as f32;
            let x = x * (element_size + padding);
            let y = y * (element_size + padding);
            commands
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: element_size })),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(x, 0.0, y),
                    ..default()
                })
                .insert(RenderLayers::from_layers(&[1, 2, 3]))
                .insert(Name::new("Plane"));
        }
    }

    let colors = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];
    let positions = [[1.0, 0.5, 2.0], [3.0, 0.5, 2.0], [5.0, 0.5, 2.0]];
    let names = ["Red", "Green", "Blue"];
    let layers = [
        RenderLayers::layer(1),
        RenderLayers::layer(2),
        RenderLayers::layer(3),
    ];
    for (color, pos, name, layer) in izip!(colors, positions, names, layers) {
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::rgb(color[0], color[1], color[2]),
            ..default()
        });

        commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: material_handle,
                transform: Transform::from_xyz(pos[0], pos[1], pos[2]),

                ..default()
            })
            .insert(layer)
            .insert(Name::new(format!("Cube {}", name)));
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
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

#[derive(AsBindGroup, TypeUuid, Clone)]
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

#[derive(AsBindGroup, TypeUuid, Clone)]
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

fn set_camera_viewports(
    windows: Res<Windows>,
    mut resize_events: EventReader<WindowResized>,
    mut red_camera: Query<
        &mut Camera,
        (With<RedCamera>, Without<BlueCamera>, Without<GreenCamera>),
    >,
    mut green_camera: Query<
        &mut Camera,
        (With<GreenCamera>, Without<RedCamera>, Without<BlueCamera>),
    >,
    mut blue_camera: Query<
        &mut Camera,
        (With<BlueCamera>, Without<RedCamera>, Without<GreenCamera>),
    >,
) {
    for resize_event in resize_events.iter() {
        if resize_event.id == WindowId::primary() {
            let window = windows.primary();
            let mut red_camera = red_camera.single_mut();
            red_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(0, 0),
                physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
                ..default()
            });

            let mut green_camera = green_camera.single_mut();
            green_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(window.physical_width() / 3, 0),
                physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
                ..default()
            });

            let mut blue_camera = blue_camera.single_mut();
            blue_camera.viewport = Some(Viewport {
                physical_position: UVec2::new(2 * window.physical_width() / 3, 0),
                physical_size: UVec2::new(window.physical_width() / 3, window.physical_height()),
                ..default()
            });
        }
    }
}
