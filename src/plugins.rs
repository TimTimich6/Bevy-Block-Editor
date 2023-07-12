use bevy::prelude::*;

use crate::setup::{Box, MainCamera, SelectedBox};
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

        info!("{:?}, {:?} ", index, index_selected.0);
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
