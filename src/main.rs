//use rand::{thread_rng, Rng};
use bevy::{prelude::*, window::WindowResolution};
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_egui::{EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
//use bevy_inspector_egui::prelude::*;


const WIN_SIZE: Vec2 = Vec2::new(600.0, 400.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "HYDRA".to_string(),
                    resolution: WindowResolution::new(WIN_SIZE.x, WIN_SIZE.y),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugin(EguiPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .insert_resource(ClearColor(Color::rgb(0.,0.,0.)))
        .add_plugin(ShapePlugin)
        .insert_resource(RapierConfiguration{
            gravity: Vec2::ZERO,
            //gravity: Vect::Y * -0.81 * 10.0,
            timestep_mode: TimestepMode::Fixed { dt: 1.0/30.0, substeps: 1 },
            physics_pipeline_active: true,
            query_pipeline_active: true,
            scaled_shape_subdivision: 10,
            force_update_from_transform_changes: false,
        })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugin(RapierDebugRenderPlugin {
            enabled: true, 
            always_on_top: false, 
            mode: DebugRenderMode::COLLIDER_SHAPES,
            style: DebugRenderStyle{
                collider_dynamic_color: [120.0, 0.5, 0.5, 1.0],
                collider_kinematic_color: [240.0, 0.5, 0.5, 1.0],
                ..Default::default()
            }
        })
        .add_startup_system(setup_graphics_system)
        .add_startup_system(create_creature)
        .run();
}

fn setup_graphics_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn create_creature(mut commands: Commands) {
    let part = shapes::RoundedPolygon{
        closed: true,
        points: vec![Vec2::new(-10.0, -10.0), Vec2::new(10.0, -10.0), Vec2::new(10.0, 10.0), Vec2::new(-10.0, 10.0)],
        radius: 2.0
    };

    let hydra = commands.spawn((
        Name::new("HYDRA".to_string()),
        RigidBody::Dynamic,
        GravityScale(0.0),
        Sleeping::disabled(),
        TransformBundle::from_transform(Transform::from_translation(Vec3 { x: 0.0, y: 0.0, z: 1.0 }))
    )).id();
    let shape_builder0 = GeometryBuilder::new().add(&part);
    let part0 = commands.spawn((
        Name::new("Part0".to_string()),
        ShapeBundle{
            path: shape_builder0.build(),
            ..Default::default()
        },
        Stroke::new(Color::YELLOW_GREEN, 2.0),
        RigidBody::Dynamic,
        Collider::round_cuboid(10.0, 10.0, 2.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3 { x: -12.0, y: 0.0, z: 1.0 }))
    )).id();
    let shape_builder1 = GeometryBuilder::new().add(&part);
    let part1 = commands.spawn((
        Name::new("Part1".to_string()),
        ShapeBundle{
            path: shape_builder1.build(),
            ..Default::default()
        },
        Stroke::new(Color::YELLOW_GREEN, 2.0),
        RigidBody::Dynamic,
        Collider::round_cuboid(10.0, 10.0, 2.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3 { x: -24.0, y: 0.0, z: 1.0 }))
    )).id();
    let shape_builder2 = GeometryBuilder::new().add(&part);
    let part2 = commands.spawn((
        Name::new("Part2".to_string()),
        ShapeBundle{
            path: shape_builder2.build(),
            ..Default::default()
        },
        Stroke::new(Color::YELLOW_GREEN, 2.0),
        RigidBody::Dynamic,
        Collider::round_cuboid(10.0, 10.0, 2.0),
        TransformBundle::from_transform(Transform::from_translation(Vec3 { x: -36.0, y: 0.0, z: 1.0 }))
    )).id();

    commands.entity(hydra).add_child(part0).add_child(part1).add_child(part2);
}
