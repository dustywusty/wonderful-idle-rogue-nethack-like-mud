use specs::{world::Builder, DispatcherBuilder, World, WorldExt};

mod components;
use components::*;

mod systems;
use systems::*;

pub struct State {
    pub ecs: World,
}

fn main() {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Player>();
    gs.ecs.register::<Name>();

    let _player = gs
        .ecs
        .create_entity()
        .with(Player {})
        .with(Name {
            name: "Dusty".to_string(),
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintingSystem, "print_sys", &[])
        .with_thread_local(NetworkingSystem)
        .build();

    loop {
        dispatcher.dispatch(&mut gs.ecs);
        gs.ecs.maintain();
    }
}
