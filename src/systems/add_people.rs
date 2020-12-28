use bevy::prelude::*;

use super::{Name, Player};

pub fn add_people(commands: &mut Commands) {
  commands
    .spawn((Player, Name("Elaina Proctor".to_string())))
    .spawn((Player, Name("Renzo Hume".to_string())))
    .spawn((Player, Name("Zayna Nieves".to_string())));
}
