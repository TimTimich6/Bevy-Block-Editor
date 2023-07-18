use bevy::prelude::*;

use crate::setup::{Box, MainCamera, SelectedBox, Shape};
pub struct CamMovementPlugin;
pub struct SelectionPlugin;

impl Plugin for CamMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(cam_rotate);
        app.add_system(cam_zoom);
    }
}

impl Plugin for SelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(select_box);
        app.add_system(delete_box);
        app.add_system(move_box);
        app.add_system(spawn_box);

        app.insert_resource(SelectedBox(0));
        // app.insert_resource()
    }
}

fn cam_rotate(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>,
    timer: Res<Time>,
) {
    let mut transform = query.single_mut();
    if keys.pressed(KeyCode::D) {
        transform.rotate_y(2. * timer.delta_seconds());
    }
    if keys.pressed(KeyCode::A) {
        transform.rotate_y(-2. * timer.delta_seconds());
    }
}

fn cam_zoom(keys: Res<Input<KeyCode>>, mut cam_query: Query<&mut Projection, With<Camera>>) {
    let proj = cam_query.single_mut();

    if let Projection::Orthographic(projection) = proj.into_inner() {
        let mut log_scale = projection.scale.ln();
        if keys.pressed(KeyCode::W) {
            log_scale -= 0.01;
        }
        if keys.pressed(KeyCode::S) {
            log_scale += 0.01;
        }
        projection.scale = log_scale.exp();
    }
}

fn select_box(
    mut commands: Commands,
    box_query: Query<
        (
            Entity,
            &mut Handle<StandardMaterial>,
            // Option<&mut SelectedBox>,
        ),
        With<Box>,
    >,

    keys: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut index_selected: ResMut<SelectedBox>,
) {
    for (index, (ent, mat)) in box_query.iter().enumerate() {
        let material = materials.get_mut(mat).unwrap();
        if keys.just_pressed(KeyCode::E) && box_query.iter().len() - 1 > index_selected.0 {
            index_selected.0 += 1;
            break;
        }
        if keys.just_pressed(KeyCode::Q) && index_selected.0 >= 1 {
            index_selected.0 -= 1;
            break;
        }
        if index_selected.0 == index {
            material.base_color = Color::GOLD;
        } else {
            material.base_color = Color::CRIMSON;
        }
    }
}

fn delete_box(
    mut commands: Commands,
    box_query: Query<
        Entity,
        // Option<&mut SelectedBox>,
        With<Box>,
    >,
    keys: Res<Input<KeyCode>>,
    index_selected: ResMut<SelectedBox>,
) {
    for (index, ent) in box_query.iter().enumerate() {
        if keys.just_pressed(KeyCode::Back) && index_selected.0 == index {
            commands.entity(ent).despawn();
            break;
        }
    }
}

fn move_box(
    mut box_query: Query<
        &mut Transform,
        // Option<&mut SelectedBox>,
        With<Box>,
    >,
    keys: Res<Input<KeyCode>>,
    index_selected: ResMut<SelectedBox>,
) {
    for (index, mut transform) in box_query.iter_mut().enumerate() {
        if keys.just_pressed(KeyCode::Space) && index_selected.0 == index {
            transform.translation += Vec3::Y;
            break;
        }
        if keys.just_pressed(KeyCode::LControl) && index_selected.0 == index {
            transform.translation += Vec3::NEG_Y;
            break;
        }
        if keys.just_pressed(KeyCode::Left) && index_selected.0 == index {
            transform.translation += Vec3::NEG_X;
            break;
        }
        if keys.just_pressed(KeyCode::Right) && index_selected.0 == index {
            transform.translation += Vec3::X;
            break;
        }
        if keys.just_pressed(KeyCode::Down) && index_selected.0 == index {
            transform.translation += Vec3::Z;
            break;
        }
        if keys.just_pressed(KeyCode::Up) && index_selected.0 == index {
            transform.translation += Vec3::NEG_Z;
            break;
        }
    }
}
fn spawn_box(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    keys: Res<Input<KeyCode>>,
    mut index_selected: ResMut<SelectedBox>,
    box_query: Query<
        Entity,
        // Option<&mut SelectedBox>,
        With<Box>,
    >,
) {
    if keys.just_pressed(KeyCode::LShift) {
        commands.spawn((
            Shape,
            PbrBundle {
                mesh: meshes.add(shape::Box::new(1f32, 1., 1f32).into()),
                material: materials.add(Color::CRIMSON.into()),
                transform: Transform::from_xyz(0., 6., 0.),
                ..default()
            },
            Box,
        ));
        index_selected.0 = box_query.iter().len();
    }
}
