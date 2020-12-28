use bevy::prelude::*;

use super::{Name, Player};

pub fn greet_people(query: Query<&Name, With<Player>>) {
  for name in query.iter() {
    println!("hello {}!", name.0);
  }
}
