use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct Message {
    message: String,
    recipient: String,
    sender: String,
}

impl Component for Message {
    type Storage = VecStorage<Self>;
}
