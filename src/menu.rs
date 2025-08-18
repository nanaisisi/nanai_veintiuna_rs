use dialoguer::{Select, theme::ColorfulTheme};
use std::io;

#[derive(Debug, Clone)]
pub enum MenuChoice {
    StartGame,
    ShowHelp,
    Quit,
}

#[derive(Debug, Clone)]
pub enum PostGameChoice {
    NextRound,
    ChangeBet,
    Settings,
    Quit,
}

impl MenuChoice {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(MenuChoice::StartGame),
            1 => Some(MenuChoice::ShowHelp),
            2 => Some(MenuChoice::Quit),
            _ => None,
        }
    }

    pub fn menu_items() -> Vec<&'static str> {
        vec!["ゲーム開始", "ヘルプ表示", "終了"]
    }
}

impl PostGameChoice {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(PostGameChoice::NextRound),
            1 => Some(PostGameChoice::ChangeBet),
            2 => Some(PostGameChoice::Settings),
            3 => Some(PostGameChoice::Quit),
            _ => None,
        }
    }

    pub fn menu_items() -> Vec<&'static str> {
        vec!["次のラウンド", "賭け金変更", "設定", "メインメニューに戻る"]
    }
}

pub fn display_help() {
    println!("\n=== ブラックジャック ヘルプ ===");
    println!("基本ルール:");
    println!("• 目標: 21に可能な限り近づけ、21を超えないようにする");
    println!("• 絵札（J、Q、K）は10ポイント");
    println!("• エース（A）は1ポイントまたは11ポイント（有利な方を自動選択）");
    println!("• ディーラーは16以下でヒット、17以上でスタンドする");
    println!("\n操作方法:");
    println!("• h/hit: カードを1枚引く");
    println!("• s/stand: 現在の手札で勝負する");
    println!("\nコマンドラインオプション:");
    println!("• cargo run -- --direct   : メニューをスキップして直接ゲーム開始");
    println!("• cargo run -- --config FILE : カスタム設定ファイルを使用");
    println!("• cargo run -- --help     : コマンドヘルプを表示");
    println!("\nEnterキーを押してメニューに戻る...");
    let mut _dummy = String::new();
    let _ = io::stdin().read_line(&mut _dummy);
}

pub fn get_user_choice() -> Result<MenuChoice, Box<dyn std::error::Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("オプションを選択してください:")
        .default(0)
        .items(&MenuChoice::menu_items())
        .interact()?;

    MenuChoice::from_index(selection)
        .ok_or_else(|| "無効な選択です".into())
}
