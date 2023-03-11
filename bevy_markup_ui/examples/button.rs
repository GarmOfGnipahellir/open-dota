use bevy::prelude::*;
use bevy_markup_ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_markup_ui::MarkupUiPlugin)
        .add_startup_system(startup)
        .run()
}

fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let scene: Handle<DynamicScene> = asset_server.load("button.html");
    commands.spawn((
        NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        scene,
    ));
}
