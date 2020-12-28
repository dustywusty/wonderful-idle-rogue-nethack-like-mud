use bevy::{ecs::prelude::*, prelude::*};
use bevy_prototype_simple_net::{
  CreateListener, ListenerConnected, ListenerError, ListenerId, NetProtocol, Port, SendSocket,
  SocketError, SocketReceive,
};

// ================================================================================================

const HOST_PORT: Port = 4000;
const LISTENER_PROTOCOL: NetProtocol = NetProtocol::Tcp;

// ================================================================================================

pub fn setup_network_listener(
  listener_id: Res<ListenerId>,
  mut listener_create: ResMut<Events<CreateListener>>,
) {
  listener_create.send(CreateListener {
    new_id: *listener_id,
    port: HOST_PORT,
    protocol: LISTENER_PROTOCOL,
  });
  info!(HOST_PORT, "listening on {:?}", "127.0.0.1");
}

// ================================================================================================

pub fn accept_connections_system(
  mut socket_send: ResMut<Events<SendSocket>>,
  mut state: Local<EventReader<ListenerConnected>>,
  listener_connected_events: Res<Events<ListenerConnected>>,
) {
  for listener_connected_event in state.iter(&listener_connected_events) {
    info!(
      "new connection from {:?}",
      listener_connected_event.socket_address
    );
    socket_send.send(SendSocket {
      id: listener_connected_event.socket_id,
      tx_data: format!("Hello!\n").into_bytes(),
    })
  }
}

// ================================================================================================

pub fn handle_error_system(
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

// ================================================================================================

pub fn handle_incoming_data(
  mut state: Local<EventReader<SocketReceive>>,
  socket_receive_events: Res<Events<SocketReceive>>,
) {
  for socket_receive_event in state.iter(&socket_receive_events) {
    info!("{:?}", socket_receive_event.rx_data);
  }
}
