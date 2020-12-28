use bevy::{
    app::{AppBuilder, Plugin},
    ecs::prelude::*,
    prelude::*,
};
pub use common::*;
pub use event::*;
pub use listeners::*;
pub use sockets::*;
use system::*;

mod common;
mod event;
mod listener;
mod listeners;
mod socket;
mod sockets;
mod system;

pub mod prelude {
    pub use crate::{
        common::{IpAddress, ListenerId, Port, SocketAddress, SocketId},
        listener::Listener,
        listeners::Listeners,
        socket::Socket,
        sockets::Sockets,
    };
}

/// Adds sockets and listeners to an app
#[derive(Default)]
pub struct NetPlugin;

impl Plugin for NetPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<OpenSocket>()
            .add_event::<SocketOpened>()
            .add_event::<SocketError>()
            .add_event::<SendSocket>()
            .add_event::<SocketSent>()
            .add_event::<SocketReceive>()
            .add_event::<CloseSocket>()
            .add_event::<SocketClosed>()
            .init_resource::<Sockets>()
            .add_system_to_stage(stage::UPDATE, open_socket_events_system.system())
            .add_system_to_stage(stage::UPDATE, socket_receive_system.system())
            .add_system_to_stage(stage::UPDATE, send_socket_events_system.system())
            .add_system_to_stage(stage::UPDATE, close_socket_events_system.system())
            .add_event::<CreateListener>()
            .add_event::<ListenerCreated>()
            .add_event::<ListenerError>()
            .add_event::<ListenerConnected>()
            .add_event::<CloseListener>()
            .add_event::<ListenerClosed>()
            .init_resource::<Listeners>()
            .add_system_to_stage(stage::UPDATE, create_listener_events_system.system())
            .add_system_to_stage(stage::UPDATE, listener_connection_system.system())
            .add_system_to_stage(stage::UPDATE, close_listener_events_system.system());
    }
}
