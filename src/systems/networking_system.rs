use super::{Name, Player};
use specs::{ReadStorage, System};

pub struct NetworkingSystem;

impl<'a> System<'a> for NetworkingSystem {
    type SystemData = (ReadStorage<'a, Name>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        println!("This just keeps running");
    }
}
