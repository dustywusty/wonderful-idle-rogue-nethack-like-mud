use super::{Name, Player};
use specs::{ReadStorage, System};

pub struct NetworkingSystem;

impl<'a> System<'a> for NetworkingSystem {
    type SystemData = (ReadStorage<'a, Name>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        println!("This just keeps running");
    }
}

// use amethyst::{
//     core::{bundle::SystemBundle, SystemDesc},
//     ecs::{
//         DispatcherBuilder, Entities, Join, LazyUpdate, Read, System, SystemData, World, Write,
//         WriteStorage,
//     },
//     network::simulation::{NetworkSimulationEvent, TransportResource},
//     prelude::Builder,
//     shrev::{EventChannel, ReaderId},
//     Result,
// };
// use log::{error, info};
// use std::str;
// use super::{Name, Player, Inbox, Message};

// #[derive(Debug)]
// pub struct NetworkReceiveBundle;

// impl<'a, 'b> SystemBundle<'a, 'b> for NetworkReceiveBundle {
//     fn build(self, world: &mut World, builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
//         builder.add(
//             NetworkReceiveSystemDesc::default().build(world),
//             "receiving_system",
//             &[],
//         );
//         Ok(())
//     }
// }

// #[derive(Default, Debug)]
// struct NetworkReceiveSystemDesc;

// impl<'a, 'b> SystemDesc<'a, 'b, NetworkReceiveSystem> for NetworkReceiveSystemDesc {
//     fn build(self, world: &mut World) -> NetworkReceiveSystem {
//         // Creates the EventChannel<NetworkEvent> managed by the ECS.
//         <NetworkReceiveSystem as System<'_>>::SystemData::setup(world);
//         // Fetch the change we just created and call `register_reader` to get a
//         // ReaderId<NetworkEvent>. This reader id is used to fetch new events from the network event
//         // channel.
//         let reader = world
//             .fetch_mut::<EventChannel<NetworkSimulationEvent>>()
//             .register_reader();
//         NetworkReceiveSystem::new(reader)
//     }
// }

// /// A simple system that receives a ton of network events.
// struct NetworkReceiveSystem {
//     reader: ReaderId<NetworkSimulationEvent>,
// }

// impl NetworkReceiveSystem {
//     pub fn new(reader: ReaderId<NetworkSimulationEvent>) -> Self {
//         Self { reader }
//     }
// }

// impl<'a> System<'a> for NetworkReceiveSystem {
//     type SystemData = (
//         Write<'a, TransportResource>,
//         Read<'a, EventChannel<NetworkSimulationEvent>>,
//         Entities<'a>,
//         Write<'a, LazyUpdate>,
//         WriteStorage<'a, Player>,
//         WriteStorage<'a, Inbox>
//     );

//     fn run(&mut self, (mut net, channel, ent, lazy, players, mut inbox): Self::SystemData) {
//         for event in channel.read(&mut self.reader) {
//             match event {
//                 NetworkSimulationEvent::Message(addr, payload) => {
//                     info!("{}: {:?}", addr, payload);

//                     let mut p: Vec<&str> = str::from_utf8(payload).unwrap().split(' ').collect();

//                     let cmd: &str = p.remove(0);
//                     let args: String = p.join(" ");

//                     (&ent, &players).join().

//                     for (ent, player) in (&ent, &players).join() {
//                         match cmd {
//                             "whisper" | "w" => {
//                                 Inbox::recieve_message(&mut inbox, ent, Message { message: args.to_string(), recipient: p.remove(0).to_string(), sender: "ME".to_string()});
//                             },
//                             _ => net.send(*addr, b"What?"),
//                         }
//                         net.send(*addr, b"\n> ");
//                     }
//                 }

//                 NetworkSimulationEvent::Connect(addr) => {
//                     info!("New client connection: {}", addr);
//                     let _player = lazy.create_entity(&ent).with(Player { addr: *addr });
//                     net.send(*addr, b"> ");
//                 }

//                 NetworkSimulationEvent::Disconnect(addr) => {
//                     info!("Client Disconnected: {}", addr);
//                 }

//                 NetworkSimulationEvent::RecvError(e) => {
//                     error!("Recv Error: {:?}", e);
//                 }

//                 NetworkSimulationEvent::SendError(_, _) => {}
//                 NetworkSimulationEvent::ConnectionError(_, _) => {}
//             }
//         }
//     }
// }
