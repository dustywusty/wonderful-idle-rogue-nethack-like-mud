use super::{Name, Player};
use specs::{ReadStorage, System};

pub struct PrintingSystem;

impl<'a> System<'a> for PrintingSystem {
    type SystemData = (ReadStorage<'a, Name>, ReadStorage<'a, Player>);

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (name, player) = data;
        for (name, player) in (&name, &player).join() {
            println!("YOOOO {:?}", name.name);
        }
    }
}
