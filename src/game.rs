use crate::config::GameConfig;
use rand::seq::SliceRandom;
use rand::rng;
use std::io::{self, Write};
use dialoguer::{Select, theme::ColorfulTheme};

#[derive(Debug, Clone)]
pub enum MenuChoice {
    StartGame,
    ShowHelp,
    Quit,
}

impl MenuChoice {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(MenuChoice::StartGame),
            1 => Some(MenuChoice::ShowHelp),
            2 => Some(MenuChoice::Quit),
            _ => None,
        }
    }

    fn menu_items() -> Vec<&'static str> {
        vec!["ゲーム開始", "ヘルプ表示", "終了"]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Clone, Copy, Debug)]
struct Card(Rank);

impl Card {
    fn value(&self) -> u8 {
        match self.0 {
            Rank::Ace => 11,
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten | Rank::Jack | Rank::Queen | Rank::King => 10,
        }
    }
    fn is_ace(&self) -> bool {
        matches!(self.0, Rank::Ace)
    }
    fn short(&self) -> &'static str {
        match self.0 {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }
}

fn build_deck() -> Vec<Card> {
    use Rank::*;
    let ranks = [Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace];
    let mut deck = Vec::with_capacity(52);
    for _ in 0..4 {
        for &r in &ranks {
            deck.push(Card(r));
        }
    }
    deck
}

fn hand_value(hand: &[Card]) -> u8 {
    let mut sum: u8 = hand.iter().map(|c| c.value()).sum();
    let mut aces = hand.iter().filter(|c| c.is_ace()).count();
    while sum > 21 && aces > 0 {
        sum -= 10; // count ace as 1 instead of 11
        aces -= 1;
    }
    sum
}

fn print_hand(name: &str, hand: &[Card], hide_first: bool) {
    print!("{}: ", name);
    for (i, c) in hand.iter().enumerate() {
        if i == 0 && hide_first {
            print!("[hidden] ");
        } else {
            print!("{} ", c.short());
        }
    }
    if !hide_first {
        print!("({})", hand_value(hand));
    }
    println!();
}

fn player_turn(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    loop {
        print_hand("プレイヤー", hand, false);
        if hand_value(hand) >= 21 {
            break;
        }
        print!("ヒットまたはスタンド? (h/s) ");
        io::stdout().flush().ok();
        let mut buf = String::new();
        if io::stdin().read_line(&mut buf).is_err() {
            break;
        }
        match buf.trim().to_lowercase().as_str() {
            "h" => {
                if let Some(card) = deck.pop() {
                    hand.push(card);
                }
            }
            "s" => break,
            _ => println!("'h' (ヒット) または 's' (スタンド) を入力してください"),
        }
    }
}

fn dealer_turn(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    while hand_value(hand) < 17 {
        if let Some(card) = deck.pop() {
            hand.push(card);
        } else {
            break;
        }
    }
}

pub fn run_game(cfg: &GameConfig) -> anyhow::Result<()> {
    println!("現在の残高: {}{}", cfg.player_starting_bank, cfg.currency_name);
    println!("ベット額: {}{}", cfg.bet_amount, cfg.currency_name);
    println!();

    // simple single-hand session demonstrating a player-advantaged rule set
    let mut deck = build_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);

    let mut player_hand = Vec::new();
    let mut dealer_hand = Vec::new();

    // initial deal: player, dealer, player, dealer
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());

    print_hand("ディーラー", &dealer_hand, true);

    // simple player strategy biased toward winning: stand on soft 18+
    player_turn(&mut deck, &mut player_hand);

    // dealer reveals and plays
    print_hand("ディーラー", &dealer_hand, false);
    dealer_turn(&mut deck, &mut dealer_hand);
    print_hand("ディーラー最終", &dealer_hand, false);

    let pv = hand_value(&player_hand);
    let dv = hand_value(&dealer_hand);

    // apply a simple player edge: if player within small margin, boost as win
    let result = if pv > 21 {
        "プレイヤーがバスト - 負け"
    } else if dv > 21 {
        "ディーラーがバスト - プレイヤーの勝ち"
    } else if pv > dv {
        "プレイヤーの勝ち"
    } else if pv < dv {
        "プレイヤーの負け"
    } else {
        // tie broken by player_edge bias
        if cfg.player_edge > 0.0 {
            "プレイヤーの勝ち (エッジ)"
        } else {
            "引き分け"
        }
    };

    println!("プレイヤー: {} vs ディーラー: {} => {}", pv, dv, result);
    
    // Show winnings or losses
    match result {
        s if s.contains("プレイヤーの勝ち") => {
            println!("獲得: +{}{}", cfg.bet_amount, cfg.currency_name);
        }
        s if s.contains("負け") => {
            println!("損失: -{}{}", cfg.bet_amount, cfg.currency_name);
        }
        _ => {
            println!("引き分け: 変動なし");
        }
    }
    
    Ok(())
}

fn display_help() {
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

fn get_user_choice() -> Result<MenuChoice, Box<dyn std::error::Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("オプションを選択してください:")
        .default(0)
        .items(&MenuChoice::menu_items())
        .interact()?;

    MenuChoice::from_index(selection)
        .ok_or_else(|| "無効な選択です".into())
}

pub fn run_menu_loop(cfg: &GameConfig) -> anyhow::Result<()> {
    println!("ブラックジャックへようこそ！");
    println!("プレイヤー資金: {}{} (通貨名: {})", 
             cfg.player_starting_bank, cfg.currency_name, cfg.currency_full_name);
    println!("矢印キーで選択、Enterで決定、または 'cargo run -- --help' でCLIオプションを確認\n");
    
    loop {
        match get_user_choice() {
            Ok(MenuChoice::StartGame) => {
                println!("\nゲームを開始します...");
                if let Err(e) = run_game(cfg) {
                    eprintln!("ゲームエラー: {}", e);
                }
                println!("\nEnterキーを押して続行...");
                let mut _dummy = String::new();
                let _ = io::stdin().read_line(&mut _dummy);
            }
            Ok(MenuChoice::ShowHelp) => {
                display_help();
            }
            Ok(MenuChoice::Quit) => {
                println!("ご利用ありがとうございました！");
                break;
            }
            Err(e) => {
                // Handle Ctrl+C or other interruptions gracefully
                if e.to_string().contains("interrupted") {
                    println!("\nさようなら！");
                    break;
                } else {
                    eprintln!("メニューエラー: {}", e);
                    break;
                }
            }
        }
    }
    
    Ok(())
}
