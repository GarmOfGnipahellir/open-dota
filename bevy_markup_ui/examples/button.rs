use bevy::prelude::*;
use bevy_ecss::{Class, StyleSheet};
use bevy_markup_ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_markup_ui::MarkupUiPlugin)
        .add_startup_system(startup)
        // .add_system(button_system)
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

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut Class, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut class, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.sections[0].value = "Press".to_string();
                *class = Class::new("press");
            }
            Interaction::Hovered => {
                text.sections[0].value = "Hover".to_string();
                *class = Class::new("hover");
            }
            Interaction::None => {
                text.sections[0].value = "Button".to_string();
                *class = Class::new("");
            }
        }
    }
}
