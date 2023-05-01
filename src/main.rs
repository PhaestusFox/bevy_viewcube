use bevy::prelude::*;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle, PickingCameraBundle};

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_editor_pls::EditorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .add_system(setup_cam.on_startup())
        .add_system(spawn_viewcube.on_startup())
        .add_system(name_cube)
        .register_type::<ViewCubeSegment>()
        .insert_resource(AmbientLight{color: Color::ANTIQUE_WHITE, brightness: 0.75})
        .run()
}

fn setup_cam(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 3., 3.))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PickingCameraBundle::default(),
    ));
}

fn spawn_viewcube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshs: ResMut<Assets<Mesh>>,
    mut matts: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshs.add(shape::Box::new(1., 1., 1.).into());
    let blank_matt = matts.add(StandardMaterial::default());
    commands
        .spawn(SpatialBundle::INHERITED_IDENTITY)
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                    transform: Transform::from_scale(Vec3::new(0.8, 0.1, 0.8))
                        .with_translation(Vec3::new(0., -0.45, 0.)),
                    ..Default::default()
                },
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
                PickableBundle::default(),
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
