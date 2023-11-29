use bevy::prelude::*;
use bevy::render::camera::Viewport;
use bevy::window::PrimaryWindow;
use hegemony::map::MapPlugin;
use hegemony::ui::GameUI;
use std::convert::*;
use std::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Hegemony"),
                        resolution: (1000.0, 720.0).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins(MapPlugin)
        .add_plugins(GameUI)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct MyMinimapCamera;

fn setup(mut commands: Commands, window: Query<&Window>) {
    let window = window.single();
    let width = window.resolution.width();
    let height = window.resolution.height();

    // calculatu viewport
    let viewport_x = (width * 0.75) as u32;
    let viewport_y = height as u32;
    println!("Window size is {}x{}", viewport_x, viewport_y);
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after / on top of other cameras
                order: 2,
                // set the viewport to a 256x256 square in the top left corner
                viewport: Some(Viewport {
                    physical_position: UVec2::new(0, 0),
                    physical_size: UVec2::new(viewport_x * 2, viewport_y * 2),
                    ..default()
                }),
                ..default()
            },
            ..default()
        },
        MyMinimapCamera,
    ));
}
