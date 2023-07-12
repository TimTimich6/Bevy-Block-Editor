use bevy::prelude::*;

use rand::distributions::{Distribution, Uniform};
pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
        app.add_system(bobble);
    }
}

#[derive(Component)]
struct Shape;

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct Box;

#[derive(Component, Debug)]
pub struct MainCamera;

#[derive(Resource, Debug)]
pub struct SelectedBox(pub usize);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            TransformBundle {
                local: Transform::from_xyz(0_f32, 0., 0.0),
                ..default()
            },
            MainCamera,
        ))
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                transform: Transform::from_xyz(5., 6., 5.)
                    .looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y),
                projection: Projection::Orthographic(OrthographicProjection {
                    scale: 2.0,
                    scaling_mode: bevy::render::camera::ScalingMode::FixedVertical(5.),
                    ..default()
                }),

                ..default()
            });
        });

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(shape::Box::new(5., 1., 5.).into()),
            material: materials.add(Color::LIME_GREEN.into()),
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
        Shape,
        Platform,
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(3., 5., 2.),
        ..default()
    });

    let mut rng = rand::thread_rng();
    for i in 1..6 {
        let rand = Uniform::from(-2..3);
        // if i == 1 {
        //     commands.spawn((
        //         Shape,
        //         PbrBundle {
        //             mesh: meshes.add(shape::Box::new(1f32, 1., 1f32).into()),
        //             material: materials.add(Color::CRIMSON.into()),
        //             transform: Transform::from_xyz(
        //                 rand.sample(&mut rng) as f32,
        //                 1.,
        //                 rand.sample(&mut rng) as f32,
        //             ),
        //             ..default()
        //         },
        //         Box,
        //         SelectedBox,
        //     ));
        // } else {
        commands.spawn((
            Shape,
            PbrBundle {
                mesh: meshes.add(shape::Box::new(1f32, 1., 1f32).into()),
                material: materials.add(Color::CRIMSON.into()),
                transform: Transform::from_xyz(
                    rand.sample(&mut rng) as f32,
                    1.,
                    rand.sample(&mut rng) as f32,
                ),
                ..default()
            },
            Box,
        ));
    }
    // }
}

/// ### Rotate

fn bobble(
    mut query: Query<&mut Transform, With<Camera>>,
    mut hit_roof: Local<bool>,
    timer: Res<Time>,
) {
    for mut transform in query.iter_mut() {
        let pY = transform.translation.y;
        if pY >= 6.2 {
            *hit_roof = true
        } else if pY <= 5.8 {
            *hit_roof = false;
        }
        let speed: f32 = if *hit_roof { -0.1 } else { 0.1 };

        // dbg!(speed);

        transform.translation += Vec3::Y * speed * timer.delta_seconds();
    }
}
