
use std::{f32::consts::PI, ops::Range, time::Duration};
use bevy::{
    prelude::*,
    ecs::{event::Event, schedule::ScheduleLabel},
    time::common_conditions::on_timer,
    utils::HashSet,
    window::PrimaryWindow,
};
use bevy_prototype_lyon::{
    entity::ShapeBundle,
    prelude::{
        tess::{geom::Rotation, math::Angle},
        *,
    },
    shapes::Polygon,
};
fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Roid UP".to_string(),
                resolution: (800.0, 600.0).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
