use bevy::prelude::*;

const PALETTE: [Color; 10] = [
    Color::RED,
    Color::YELLOW,
    Color::WHITE,
    Color::BEIGE,
    Color::CYAN,
    Color::CRIMSON,
    Color::NAVY,
    Color::AZURE,
    Color::GREEN,
    Color::BLACK,
];

pub struct GameUI;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_game_ui)
            .add_systems(Update, update_game_ui);
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(25.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    left: Val::Vw(50.0),
                    flex_wrap: FlexWrap::Wrap,
                    ..default()
                },
                background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|builder| {
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vw(30.),
                        height: Val::Vh(30.),
                        border: UiRect::all(Val::VMin(0.5)),
                        ..default()
                    },
                    background_color: PALETTE[2].into(),
                    border_color: PALETTE[9].into(),
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn((TextBundle {
                        text: Text::from_section(
                            "Grid Info!",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    },));
                });

            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vw(30.),
                        height: Val::Vh(30.),
                        border: UiRect::all(Val::VMin(0.5)),
                        ..default()
                    },
                    background_color: PALETTE[3].into(),
                    border_color: PALETTE[9].into(),
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn((TextBundle {
                        text: Text::from_section(
                            "Troop Info!",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    },));
                });
            builder
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Vw(30.),
                        height: Val::Vh(30.),
                        border: UiRect::all(Val::VMin(0.5)),
                        ..default()
                    },
                    background_color: PALETTE[4].into(),
                    border_color: PALETTE[9].into(),
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn((TextBundle {
                        text: Text::from_section(
                            "Other info!",
                            TextStyle {
                                font_size: 32.0,
                                color: Color::BLACK,
                                ..default()
                            },
                        ),
                        ..default()
                    },));
                });
        });
}

fn update_game_ui() {}
