use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub currency_name: String,
    pub currency_full_name: String,
    pub player_starting_bank: u32,
    pub bet_amount: u32,
    /// house edge bias: positive gives advantage to player (for "winning" Blackjack)
    pub player_edge: f32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            currency_name: "np".to_string(),
            currency_full_name: "nanai points".to_string(),
            player_starting_bank: 1000,
            bet_amount: 50,
            player_edge: 0.0,
        }
    }
}

impl GameConfig {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let s = fs::read_to_string(path)?;
        let cfg: Self = toml::from_str(&s)?;
        Ok(cfg)
    }
}
