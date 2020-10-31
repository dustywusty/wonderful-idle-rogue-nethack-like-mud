use amethyst::ecs::{Component, DenseVecStorage, FlaggedStorage};

pub struct Name {}

impl Component for Name {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
