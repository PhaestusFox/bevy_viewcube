use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle, Selection};
use bevy_mod_outline::{OutlineBundle, OutlinePlugin, OutlineVolume};

mod player;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(OutlinePlugin)
        .add_system(setup_cam.on_startup())
        .add_system(spawn_viewcube.on_startup())
        .add_system(name_cube)
        .register_type::<ViewCubeSegment>()
        .insert_resource(AmbientLight{color: Color::ANTIQUE_WHITE, brightness: 0.75})
        .add_system(cube_events)
        .add_system(rotation_cube)
        .add_plugin(player::PlayerPlugin)
        .add_system(outline_selected)
        .run()
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 3., 0.))
                .looking_at(Vec3::ZERO, -Vec3::Z),
            ..Default::default()
        },
        PickingCameraBundle::default(),
        player::LookData::default()
    ));
}

#[derive(Debug, Component)]
struct ViewCube;

fn spawn_viewcube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut matts: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshs.add(shape::Box::new(1., 1., 1.).into());
    let blank_matt = matts.add(StandardMaterial::default());
    commands
        .spawn((SpatialBundle::INHERITED_IDENTITY, ViewCube))
        .with_children(|commands| {
            // top
            {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/top.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.8))
                        .with_translation(Vec3::new(0., 0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Top,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.8))
                        .with_translation(Vec3::new(-0.45, 0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.8))
                        .with_translation(Vec3::new(0.45, 0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopRight,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.1))
                        .with_translation(Vec3::new(0., 0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopBack,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.1))
                        .with_translation(Vec3::new(0., 0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopFront,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(-0.45, 0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopBackLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(0.45, 0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopBackRight,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(-0.45, 0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopFrontLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(0.45, 0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::TopFrontRight,
            ));
        }
            // bottom
            {
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/bottom.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(-0.8, -0.1, 0.8))
                        .with_translation(Vec3::new(0., -0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Bottom,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.8))
                        .with_translation(Vec3::new(-0.45, -0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.8))
                        .with_translation(Vec3::new(0.45, -0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomRight,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.1))
                        .with_translation(Vec3::new(0., -0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomBack,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.1))
                        .with_translation(Vec3::new(0., -0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomFront,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(-0.45, -0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomBackLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(0.45, -0.45, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomBackRight,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(-0.45, -0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomFrontLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.1, 0.1))
                        .with_translation(Vec3::new(0.45, -0.45, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BottomFrontRight,
            ));
        }
            // middle
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/front.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(-0.8, 0.8, -0.1))
                        .with_translation(Vec3::new(0., 0.0, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Front,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.8, 0.1))
                        .with_translation(Vec3::new(-0.45, 0.0, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::FrontLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.8, 0.1))
                        .with_translation(Vec3::new(0.45, 0.0, 0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::FrontRight,
            ));

            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/back.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(0.8, 0.8, 0.1))
                        .with_translation(Vec3::new(0., 0.0, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Back,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.8, 0.1))
                        .with_translation(Vec3::new(-0.45, 0.0, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BackLeft,
            ));
            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: blank_matt.clone(),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.8, 0.1))
                        .with_translation(Vec3::new(0.45, 0.0, -0.45)),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::BackRight,
            ));

            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/left.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(0.1, 0.8, 0.8))
                        .with_translation(Vec3::new(-0.45, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_x(-90.0f32.to_radians())),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Left,
            ));

            commands.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: matts.add(StandardMaterial {
                        base_color_texture: Some(asset_server.load("cube/right.png")),
                        ..Default::default()
                    }),
                    transform: Transform::from_scale(Vec3::new(-0.1, 0.8, -0.8))
                        .with_translation(Vec3::new(0.45, 0.0, 0.0))
                        .with_rotation(Quat::from_rotation_x(90.0f32.to_radians())),
                    ..Default::default()
                },
                PickableBundle::default(), OutlineBundle {
        outline: OutlineVolume {
            visible: false,
            width: 15.,
            colour: Color::GOLD,
        },
        ..Default::default()
    },
                ViewCubeSegment::Right,
            ));
        });
}

fn name_cube(
    mut commands: Commands,
    query: Query<(Entity, &ViewCubeSegment), (Changed<ViewCubeSegment>, Without<Name>)>,
) {
    for (entity, seg) in &query {
        commands
            .entity(entity)
            .insert(Name::new(format!("{:?}", seg)));
    }
}

fn rotation_cube(
    input: Res<Input<KeyCode>>,
    mut view_cube: Query<&mut Transform, With<player::LookData>>,
) {   
    let mut cube = view_cube.single_mut();
    if input.just_pressed(KeyCode::Space) {
        let rotation = cube.rotation.to_euler(EulerRot::XYZ);
        println!("Quat::from_euler(EulerRot::XYZ, {}, {}, {})", rotation.0, rotation.1, rotation.2);
    }
    for key in input.get_just_pressed() {
        match key {
            KeyCode::Numpad1 => cube.rotate(Quat::from_rotation_x(-45.0f32.to_radians())),
            KeyCode::Numpad2 => cube.rotate(Quat::from_rotation_y(-45.0f32.to_radians())),
            KeyCode::Numpad3 => cube.rotate(Quat::from_rotation_z(-45.0f32.to_radians())),
            KeyCode::Numpad7 => cube.rotate(Quat::from_rotation_x(45.0f32.to_radians())),
            KeyCode::Numpad8 => cube.rotate(Quat::from_rotation_y(45.0f32.to_radians())),
            KeyCode::Numpad9 => cube.rotate(Quat::from_rotation_z(45.0f32.to_radians())),
            _ => {}
        }
    }
}

fn cube_events(
    query: Query<(&ViewCubeSegment, &Interaction), Changed<Interaction>>,
    mut view_cube: Query<&mut Transform, With<ViewCube>>
) {
    let mut cube = view_cube.single_mut();
    for (segment, interaction) in &query {
        if let Interaction::Clicked = interaction {
            cube.rotation = segment.get_rotation();
        }
    }
}

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
enum ViewCubeSegment {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
    TopLeft,
    TopRight,
    TopFront,
    TopBack,
    TopFrontLeft,
    TopFrontRight,
    TopBackLeft,
    TopBackRight,
    FrontLeft,
    FrontRight,
    BackLeft,
    BackRight,
    BottomLeft,
    BottomRight,
    BottomFront,
    BottomBack,
    BottomFrontLeft,
    BottomFrontRight,
    BottomBackLeft,
    BottomBackRight,
}

impl ViewCubeSegment {
    fn get_rotation(&self) -> Quat {
        match self {
            ViewCubeSegment::Top => Quat::from_euler(EulerRot::XYZ, 0., 0., 0.),
            ViewCubeSegment::Bottom => Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 0., 180.0f32.to_radians()),
            ViewCubeSegment::Left => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 90.0f32.to_radians(), -90.0f32.to_radians())},
            ViewCubeSegment::Right => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), -90.0f32.to_radians(), 90.0f32.to_radians())},
            ViewCubeSegment::Front => {Quat::from_euler(EulerRot::XYZ, -90.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::Back => {Quat::from_euler(EulerRot::XYZ, 90.0f32.to_radians(), 0.0f32.to_radians(), 180.0f32.to_radians())},
            ViewCubeSegment::TopLeft => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 0.0f32.to_radians(), -45.0f32.to_radians())},
            ViewCubeSegment::TopRight => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 0.0f32.to_radians(), 45.0f32.to_radians())},
            ViewCubeSegment::TopFront => {Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::TopBack => Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians()),
            ViewCubeSegment::TopFrontLeft => {Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), 45.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::TopFrontRight => {Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), -45.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::TopBackLeft => {Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), -135.0f32.to_radians(), 45.0f32.to_radians())},
            ViewCubeSegment::TopBackRight => {Quat::from_euler(EulerRot::XYZ, -45.0f32.to_radians(), -135.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::FrontLeft => {Quat::from_euler(EulerRot::XYZ, -90.0f32.to_radians(), 45.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::FrontRight => {Quat::from_euler(EulerRot::XYZ, -90.0f32.to_radians(), -45.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::BackLeft => {Quat::from_euler(EulerRot::XYZ, 90.0f32.to_radians(), 45.0f32.to_radians(), 180.0f32.to_radians())},
            ViewCubeSegment::BackRight => {Quat::from_euler(EulerRot::XYZ, 90.0f32.to_radians(), -45.0f32.to_radians(), 180.0f32.to_radians())},
            ViewCubeSegment::BottomLeft => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 0.0f32.to_radians(), 225.0f32.to_radians())},
            ViewCubeSegment::BottomRight => {Quat::from_euler(EulerRot::XYZ, 0.0f32.to_radians(), 0.0f32.to_radians(), 135.0f32.to_radians())},
            ViewCubeSegment::BottomFront => {Quat::from_euler(EulerRot::XYZ, -135.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::BottomBack => {Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 180.0f32.to_radians())},
            ViewCubeSegment::BottomFrontLeft => {warn!("Not Done"); Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::BottomFrontRight => {warn!("Not Done"); Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::BottomBackLeft => {warn!("Not Done"); Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
            ViewCubeSegment::BottomBackRight => {warn!("Not Done"); Quat::from_euler(EulerRot::XYZ, 45.0f32.to_radians(), 0.0f32.to_radians(), 0.0f32.to_radians())},
        }
    }
}

fn outline_selected(
    mut query: Query<(&mut OutlineVolume, &Selection), Changed<Selection>>
) {
    for (mut outline, selection) in &mut query {
        outline.visible = selection.selected();
    }
}