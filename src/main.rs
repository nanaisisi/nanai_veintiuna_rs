mod config;
mod game;

use config::GameConfig;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Keep main minimal: load config and run a single game session
    let cfg = GameConfig::load(Path::new("game_config.toml"))?;
    game::run_game(&cfg)?;
    Ok(())
}
