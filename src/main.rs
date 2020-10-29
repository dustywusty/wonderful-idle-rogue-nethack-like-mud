use std::time::Duration;

use amethyst::{
    core::frame_limiter::FrameRateLimitStrategy, network::simulation::tcp::TcpNetworkBundle,
    prelude::*, utils::application_root_dir, Result,
};

use std::net::TcpListener;

mod systems;
use systems::*;

pub struct GameState;
impl SimpleState for GameState {}

fn main() -> Result<()> {
    amethyst::start_logger(Default::default());

    let listener = TcpListener::bind("0.0.0.0:3457")?;
    listener.set_nonblocking(true)?;

    let assets_dir = application_root_dir()?.join("examples/net_server");

    let game_data = GameDataBuilder::default()
        .with_bundle(TcpNetworkBundle::new(Some(listener), 2048))?
        .with_bundle(SpamReceiveBundle)?;

    let mut game = Application::build(assets_dir, GameState)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            60,
        )
        .build(game_data)?;
    game.run();

    Ok(())
}
