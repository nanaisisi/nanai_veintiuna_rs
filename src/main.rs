mod config;
mod game;

use clap::{Arg, Command};
use config::GameConfig;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let matches = Command::new("blackjack") // "nanai_veintiuna_rs" | "blackjack" | "veintiuna"
        .version("0.1.0")
        .about("メニュー駆動式ブラックジャックゲーム")
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("ファイル")
            .help("カスタム設定ファイルを指定")
            .default_value("game_config.toml"))
        .arg(Arg::new("direct")
            .short('d')
            .long("direct")
            .action(clap::ArgAction::SetTrue)
            .help("メニューをスキップして直接ゲーム開始"))
        .get_matches();

    let config_path = matches.get_one::<String>("config").unwrap();
    let cfg = GameConfig::load(Path::new(config_path))?;

    if matches.get_flag("direct") {
        // Direct game mode
        game::run_game(&cfg)?;
    } else {
        // Menu mode (default)
        game::run_menu_loop(&cfg)?;
    }
    
    Ok(())
}
