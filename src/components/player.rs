use amethyst::ecs::{Component, VecStorage};
use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq)]
pub struct Player {
    pub socket: SocketAddr,
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}
