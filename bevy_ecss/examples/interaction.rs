use bevy::prelude::*;
use bevy_ecss::prelude::{EcssPlugin, StyleSheet};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EcssPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::width(Val::Percent(100.0)),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            StyleSheet::new(asset_server.load("sheets/interaction.css")),
        ))
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle::default())
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Button",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}
