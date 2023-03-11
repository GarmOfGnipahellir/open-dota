mod main_menu;

use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    QuinnetClientPlugin,
};

use open_dota_server::{ClientMessage, ServerMessage};

#[derive(States, Debug, Default, PartialEq, Eq, Hash, Clone)]
pub enum ClientState {
    #[default]
    MainMenu,
    InGame,
}

fn main() {
    App::new()
        .add_state::<ClientState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(QuinnetClientPlugin::default())
        .add_plugin(main_menu::MainMenuPlugin)
        .add_startup_system(startup)
        .add_system(handle_server_messages)
        .run();
}

fn startup(mut commands: Commands, mut client: ResMut<Client>) {
    commands.spawn(Camera2dBundle::default());

    client
        .open_connection(
            ConnectionConfiguration::from_ips(
                IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                6000,
                IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
                0,
            ),
            CertificateVerificationMode::SkipVerification,
        )
        .unwrap();
}

fn handle_server_messages(mut client: ResMut<Client>) {
    while let Ok(Some(message)) = client.connection_mut().receive_message::<ServerMessage>() {
        match message {
            ServerMessage::InitClient => info!("Connected to server!"),
            ServerMessage::ChatMessage { message } => info!("Chat message: '{message}'"),
        }
    }
}
