use std::net::{IpAddr, Ipv4Addr};

use bevy::prelude::*;
use bevy_quinnet::client::{
    certificate::CertificateVerificationMode, connection::ConnectionConfiguration, Client,
    QuinnetClientPlugin,
};

use open_dota_server::{ClientMessage, ServerMessage};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(QuinnetClientPlugin::default())
        .add_startup_system(startup)
        .run();
}

fn startup(mut client: ResMut<Client>) {
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
            ServerMessage::InitClient => todo!(),
            ServerMessage::ChatMessage { message } => todo!(),
        }
    }
}
