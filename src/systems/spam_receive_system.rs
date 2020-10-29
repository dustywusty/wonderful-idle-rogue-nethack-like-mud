use amethyst::{
    core::{bundle::SystemBundle, SystemDesc},
    ecs::{DispatcherBuilder, Read, System, SystemData, World, Write},
    network::simulation::{NetworkSimulationEvent, TransportResource},
    shrev::{EventChannel, ReaderId},
    Result,
};

use log::{error, info};

#[derive(Debug)]
pub struct SpamReceiveBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SpamReceiveBundle {
    fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        builder.add(
            SpamReceiveSystemDesc::default().build(world),
            "receiving_system",
            &[],
        );
        Ok(())
    }
}

#[derive(Default, Debug)]
struct SpamReceiveSystemDesc;

impl<'a, 'b> SystemDesc<'a, 'b, SpamReceiveSystem> for SpamReceiveSystemDesc {
    fn build(self, world: &mut World) -> SpamReceiveSystem {
        // Creates the EventChannel<NetworkEvent> managed by the ECS.
        <SpamReceiveSystem as System<'_>>::SystemData::setup(world);
        // Fetch the change we just created and call `register_reader` to get a
        // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
        // channel.
        let reader = world
            .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
            .register_reader();
        SpamReceiveSystem::new(reader)
    }
}

/// A simple system that receives a ton of network events.
struct SpamReceiveSystem {
    reader: ReaderId<NetworkSimulationEvent>,
}

impl SpamReceiveSystem {
    pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
        Self { reader }
    }
}

impl<'a> System<'a> for SpamReceiveSystem {
    type SystemData = (
        Write<'a, TransportResource>,
        Read<'a, EventChannel<NetworkSimulationEvent>>,
    );

    fn run(&mut self, (mut net, channel): Self::SystemData) {
        for event in channel.read(&mut self.reader) {
            match event {
                NetworkSimulationEvent::Message(addr, payload) => {
                    info!("{}: {:?}", addr, payload);
                    // In a typical client/server simulation, both the client and the server will
                    // be exchanging messages at a constant rate. Laminar makes use of this by
                    // packaging message acks with the next sent message. Therefore, in order for
                    // reliability to work properly, we'll send a generic "ok" response.
                    net.send(*addr, b"ok");
                }
                NetworkSimulationEvent::Connect(addr) => info!("New client connection: {}", addr),
                NetworkSimulationEvent::Disconnect(addr) => {
                    info!("Client Disconnected: {}", addr);
                }
                NetworkSimulationEvent::RecvError(e) => {
                    error!("Recv Error: {:?}", e);
                }
                _ => {}
            }
        }
    }
}
