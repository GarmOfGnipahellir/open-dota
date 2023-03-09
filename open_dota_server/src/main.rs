use std::net::{IpAddr, Ipv4Addr};

use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_quinnet::server::{
    certificate::CertificateRetrievalMode, QuinnetServerPlugin, Server, ServerConfiguration,
};

use open_dota_server::{ClientMessage, ServerMessage};

fn main() {
    App::default()
        .add_plugins(DefaultPlugins)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(QuinnetServerPlugin::default())
        .add_startup_system(startup)
        .add_system(handle_client_messages)
        .run();
}

fn startup(mut server: ResMut<Server>) {
    server
        .start_endpoint(
            ServerConfiguration::from_ip(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 6000),
            CertificateRetrievalMode::GenerateSelfSigned {
                server_hostname: "127.0.0.1".to_string(),
            },
        )
        .unwrap();
}

fn handle_client_messages(mut server: ResMut<Server>) {
    let endpoint = server.endpoint_mut();
    for client_id in endpoint.clients() {
        while let Some(message) = endpoint.try_receive_message_from::<ClientMessage>(client_id) {
            match message {
                ClientMessage::Join => {
                    endpoint
                        .send_message(client_id, ServerMessage::InitClient)
                        .unwrap();
                }
                ClientMessage::Leave => endpoint.disconnect_client(client_id).unwrap(),
                ClientMessage::ChatMessage { message } => endpoint
                    .send_group_message(
                        endpoint.clients().iter(),
                        ServerMessage::ChatMessage { message },
                    )
                    .unwrap(),
            }
        }
    }
}
