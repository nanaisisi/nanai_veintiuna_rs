use crate::config::GameConfig;
use rand::seq::SliceRandom;
use rand::rng;
use std::io;
use dialoguer::{Select, theme::ColorfulTheme};

#[derive(Debug, Clone)]
pub enum MenuChoice {
    StartGame,
    ShowHelp,
    Quit,
}

#[derive(Debug, Clone)]
pub enum GameAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

#[derive(Debug, Clone)]
pub enum PostGameChoice {
    NextRound,
    ChangeBet,
    Settings,
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

impl GameAction {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(GameAction::Hit),
            1 => Some(GameAction::Stand),
            2 => Some(GameAction::DoubleDown),
            3 => Some(GameAction::Split),
            4 => Some(GameAction::Surrender),
            _ => None,
        }
    }

    fn menu_items(can_double: bool, can_split: bool, can_surrender: bool) -> Vec<&'static str> {
        let mut items = vec!["ヒット（カードを引く）", "スタンド（現在の手札で勝負）"];
        
        if can_double {
            items.push("ダブルダウン（ベット2倍、1枚のみ引く）");
        }
        if can_split {
            items.push("スプリット（手札を分割）");
        }
        if can_surrender {
            items.push("サレンダー（降参、半額返却）");
        }
        
        items
    }

    fn get_valid_actions(can_double: bool, can_split: bool, can_surrender: bool) -> Vec<Self> {
        let mut actions = vec![GameAction::Hit, GameAction::Stand];
        
        if can_double {
            actions.push(GameAction::DoubleDown);
        }
        if can_split {
            actions.push(GameAction::Split);
        }
        if can_surrender {
            actions.push(GameAction::Surrender);
        }
        
        actions
    }
}

impl PostGameChoice {
    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(PostGameChoice::NextRound),
            1 => Some(PostGameChoice::ChangeBet),
            2 => Some(PostGameChoice::Settings),
            3 => Some(PostGameChoice::Quit),
            _ => None,
        }
    }

    fn menu_items() -> Vec<&'static str> {
        vec!["次のラウンド", "賭け金変更", "設定", "メインメニューに戻る"]
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
            Rank::Jack => "J(10)",
            Rank::Queen => "Q(10)",
            Rank::King => "K(10)",
            Rank::Ace => "A(11)",
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
    
    // Calculate how many aces are being counted as 1
    let mut sum: u8 = hand.iter().map(|c| c.value()).sum();
    let mut aces = hand.iter().filter(|c| c.is_ace()).count();
    let mut aces_as_1 = 0;
    while sum > 21 && aces > 0 {
        sum -= 10;
        aces -= 1;
        aces_as_1 += 1;
    }
    
    let mut current_aces_as_1 = aces_as_1;
    
    for (i, c) in hand.iter().enumerate() {
        if i == 0 && hide_first {
            print!("[hidden] ");
        } else {
            if c.is_ace() {
                if current_aces_as_1 > 0 {
                    print!("A(1) ");
                    current_aces_as_1 -= 1;
                } else {
                    print!("A(11) ");
                }
            } else {
                print!("{} ", c.short());
            }
        }
    }
    if !hide_first {
        print!("({})", hand_value(hand));
    }
    println!();
}

#[derive(Debug, Clone)]
pub enum PlayerActionResult {
    Continue,
    Stand,
    DoubleDown,
    Split(Vec<Card>, Vec<Card>), // Two split hands
    Surrender,
}

fn can_double_down(hand: &[Card]) -> bool {
    hand.len() == 2
}

fn can_split(hand: &[Card]) -> bool {
    hand.len() == 2 && hand[0].value() == hand[1].value()
}

fn can_surrender(hand: &[Card]) -> bool {
    hand.len() == 2
}

fn player_turn(deck: &mut Vec<Card>, hand: &mut Vec<Card>, is_first_turn: bool) -> anyhow::Result<PlayerActionResult> {
    loop {
        print_hand("プレイヤー", hand, false);
        if hand_value(hand) >= 21 {
            return Ok(PlayerActionResult::Stand);
        }
        
        let can_double = is_first_turn && can_double_down(hand);
        let can_split_hand = is_first_turn && can_split(hand);
        let can_surrender_hand = is_first_turn && can_surrender(hand);
        
        let valid_actions = GameAction::get_valid_actions(can_double, can_split_hand, can_surrender_hand);
        let menu_items = GameAction::menu_items(can_double, can_split_hand, can_surrender_hand);
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("次のアクションを選択してください:")
            .default(0)
            .items(&menu_items)
            .interact()?;

        if let Some(action) = valid_actions.get(selection) {
            match action {
                GameAction::Hit => {
                    if let Some(card) = deck.pop() {
                        hand.push(card);
                        println!("カードを引きました: {}", card.short());
                        // Continue the loop
                    }
                }
                GameAction::Stand => {
                    println!("スタンドしました");
                    return Ok(PlayerActionResult::Stand);
                }
                GameAction::DoubleDown => {
                    if let Some(card) = deck.pop() {
                        hand.push(card);
                        println!("ダブルダウン: カードを1枚引きました: {}", card.short());
                        return Ok(PlayerActionResult::DoubleDown);
                    } else {
                        println!("デッキが不足しています");
                        return Ok(PlayerActionResult::Stand);
                    }
                }
                GameAction::Split => {
                    if hand.len() == 2 && deck.len() >= 2 {
                        let second_card = hand.pop().unwrap();
                        let first_card = hand[0];
                        
                        // Create two new hands
                        let mut hand1 = vec![first_card];
                        let mut hand2 = vec![second_card];
                        
                        // Deal one card to each hand
                        if let Some(card) = deck.pop() {
                            hand1.push(card);
                        }
                        if let Some(card) = deck.pop() {
                            hand2.push(card);
                        }
                        
                        println!("スプリットしました");
                        return Ok(PlayerActionResult::Split(hand1, hand2));
                    } else {
                        println!("スプリットできません");
                    }
                }
                GameAction::Surrender => {
                    println!("サレンダーしました（半額返却）");
                    return Ok(PlayerActionResult::Surrender);
                }
            }
        } else {
            println!("無効な選択です");
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

    let mut deck = build_deck();
    let mut rng = rng();
    deck.shuffle(&mut rng);

    let mut player_hands = Vec::new();
    let mut dealer_hand = Vec::new();

    // initial deal: player, dealer, player, dealer
    let mut initial_hand = Vec::new();
    initial_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    initial_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());

    player_hands.push(initial_hand);

    print_hand("ディーラー", &dealer_hand, true);

    let mut total_bet = cfg.bet_amount;
    let mut results: Vec<PlayerActionResult> = Vec::new();

    // Process each player hand (initially just one, but can become multiple with splits)
    let mut hand_index = 0;
    while hand_index < player_hands.len() {
        let mut current_hand = player_hands[hand_index].clone();
        
        println!("\n--- 手札 {} ---", hand_index + 1);
        
        // Check if this is the first action for this hand (for split/double/surrender eligibility)
        let is_first_action = current_hand.len() == 2;
        
        match player_turn(&mut deck, &mut current_hand, is_first_action)? {
            PlayerActionResult::Continue | PlayerActionResult::Stand => {
                player_hands[hand_index] = current_hand;
                hand_index += 1; // Move to next hand
            }
            PlayerActionResult::DoubleDown => {
                total_bet += cfg.bet_amount;
                player_hands[hand_index] = current_hand;
                println!("ダブルダウンで総ベット額: {}{}", total_bet, cfg.currency_name);
                hand_index += 1; // Move to next hand
            }
            PlayerActionResult::Split(hand1, hand2) => {
                total_bet += cfg.bet_amount;
                player_hands[hand_index] = hand1;
                player_hands.push(hand2);
                println!("スプリットで総ベット額: {}{}", total_bet, cfg.currency_name);
                // Don't increment hand_index - continue with current hand (hand1)
            }
            PlayerActionResult::Surrender => {
                let refund = cfg.bet_amount / 2;
                println!("サレンダー - 返却額: {}{}", refund, cfg.currency_name);
                
                // Post-game menu
                let continue_playing = show_post_game_menu(cfg)?;
                
                if continue_playing {
                    return run_game(cfg);
                }
                
                return Ok(());
            }
        }
    }

    // dealer reveals and plays
    println!("\n--- ディーラーのターン ---");
    print_hand("ディーラー", &dealer_hand, false);
    dealer_turn(&mut deck, &mut dealer_hand);
    print_hand("ディーラー最終", &dealer_hand, false);

    let dv = hand_value(&dealer_hand);

    // Evaluate each hand
    let mut total_winnings = 0i32;
    
    for (i, hand) in player_hands.iter().enumerate() {
        let pv = hand_value(hand);
        println!("\n--- 手札 {} の結果 ---", i + 1);
        print_hand(&format!("プレイヤー手札{}", i + 1), hand, false);
        
        let result = if pv > 21 {
            "バスト - 負け"
        } else if dv > 21 {
            "ディーラーがバスト - 勝ち"
        } else if pv > dv {
            "勝ち"
        } else if pv < dv {
            "負け"
        } else {
            // tie broken by player_edge bias
            if cfg.player_edge > 0.0 {
                "勝ち (エッジ)"
            } else {
                "引き分け"
            }
        };

        println!("プレイヤー: {} vs ディーラー: {} => {}", pv, dv, result);
        
        let hand_bet = cfg.bet_amount;
        match result {
            s if s.contains("勝ち") => {
                total_winnings += hand_bet as i32;
                println!("獲得: +{}{}", hand_bet, cfg.currency_name);
            }
            s if s.contains("負け") => {
                total_winnings -= hand_bet as i32;
                println!("損失: -{}{}", hand_bet, cfg.currency_name);
            }
            _ => {
                println!("引き分け: 変動なし");
            }
        }
    }

    println!("\n--- 総合結果 ---");
    println!("総ベット額: {}{}", total_bet, cfg.currency_name);
    if total_winnings > 0 {
        println!("総獲得: +{}{}", total_winnings, cfg.currency_name);
    } else if total_winnings < 0 {
        println!("総損失: {}{}", -total_winnings, cfg.currency_name);
    } else {
        println!("総合結果: 引き分け");
    }
    
    // Post-game menu
    let continue_playing = show_post_game_menu(cfg)?;
    
    if continue_playing {
        return run_game(cfg);
    }
    
    Ok(())
}

fn show_post_game_menu(cfg: &GameConfig) -> anyhow::Result<bool> {
    loop {
        println!();
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("次のアクションを選択してください:")
            .default(0)
            .items(&PostGameChoice::menu_items())
            .interact()?;

        match PostGameChoice::from_index(selection) {
            Some(PostGameChoice::NextRound) => {
                println!("新しいラウンドを開始します...");
                return Ok(true); // Continue playing
            }
            Some(PostGameChoice::ChangeBet) => {
                println!("賭け金変更機能は今後実装予定です");
                println!("現在のベット額: {}{}", cfg.bet_amount, cfg.currency_name);
            }
            Some(PostGameChoice::Settings) => {
                println!("設定機能は今後実装予定です");
                println!("設定ファイル: game_config.toml で設定を変更できます");
            }
            Some(PostGameChoice::Quit) => {
                println!("メインメニューに戻ります");
                return Ok(false); // Stop playing
            }
            None => {
                println!("無効な選択です");
            }
        }
    }
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
                match run_game(cfg) {
                    Ok(()) => {
                        // ゲームが正常終了（メインメニューに戻る選択）
                    }
                    Err(e) => {
                        if e.to_string().contains("interrupted") {
                            println!("\nゲームが中断されました");
                        } else {
                            eprintln!("ゲームエラー: {}", e);
                        }
                    }
                }
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
