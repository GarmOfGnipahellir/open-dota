use bevy::prelude::*;

use crate::ClientState;

fn setup_main_menu(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::width(Val::Px(200.0)),
                        border: UiRect::all(Val::Px(2.0)),
                        ..Default::default()
                    },
                    background_color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                });
        });
}

fn cleanup_main_menu() {}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_main_menu.in_schedule(OnEnter(ClientState::MainMenu)))
            .add_system(cleanup_main_menu.in_schedule(OnExit(ClientState::MainMenu)));
    }
}
