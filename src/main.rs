use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    ecs::prelude::*,
    log::*,
    prelude::*,
    utils::Duration,
};
use bevy_prototype_simple_net::{
    CreateListener, ListenerConnected, ListenerError, ListenerId, NetPlugin, NetProtocol, Port,
    SendSocket,
};

// ================================================================================================
// ================================================================================================

const HOST_PORT: Port = 4000;
const LISTENER_PROTOCOL: NetProtocol = NetProtocol::Tcp;

// ================================================================================================
// ================================================================================================

fn main() {
    App::build()
        .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_millis(500)))
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(NetPlugin)
        .add_resource(ListenerId::new())
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::UPDATE, accept_connections_system.system())
        .add_system_to_stage(stage::UPDATE, handle_error_system.system())
        .run();
}

// ================================================================================================
// ================================================================================================

fn setup(listener_id: Res<ListenerId>, mut listener_create: ResMut<Events<CreateListener>>) {
    listener_create.send(CreateListener {
        new_id: *listener_id,
        port: HOST_PORT,
        protocol: LISTENER_PROTOCOL,
    });
    info!(HOST_PORT, "connected to {:?}", "127.0.0.1");
}

// ================================================================================================
// ================================================================================================

fn accept_connections_system(
    mut socket_send: ResMut<Events<SendSocket>>,
    mut state: Local<EventReader<ListenerConnected>>,
    listener_connected_events: Res<Events<ListenerConnected>>,
) {
    // Accept connections and respond with message
    for listener_connected_event in state.iter(&listener_connected_events) {
        println!(
            "Received a new connection from {:?}",
            listener_connected_event.socket_address
        );
        socket_send.send(SendSocket {
            id: listener_connected_event.socket_id,
            tx_data: format!("Hello!\n").into_bytes(),
        })
    }
}

fn handle_error_system(
    mut state: Local<EventReader<ListenerError>>,
    listener_error_events: Res<Events<ListenerError>>,
) {
    for listener_error_event in state.iter(&listener_error_events) {
        eprintln!(
            "Listener error (ID: {:?}): \"{:?}\"",
            listener_error_event.id, listener_error_event.err
        );
    }
}
