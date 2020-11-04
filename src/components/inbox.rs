use amethyst::{
    ecs::{Component, DenseVecStorage, FlaggedStorage, WriteStorage, VecStorage},
    ecs::prelude::Entity
};
use super::{Message, Player};

pub struct Inbox { 
    pub messages: Vec<Message>
}

impl Component for Inbox {
    type Storage = VecStorage<Self>;
}

impl Inbox {
    pub fn recieve_message(store: &mut WriteStorage<Inbox>, entity: Entity, message: Message) {
        println!("Got message {} for entity {}", message.message, entity.id());
    }
}