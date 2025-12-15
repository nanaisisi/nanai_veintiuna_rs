use crate::config::GameConfig;
use crate::card::{build_deck, shuffle_deck, hand_value, is_blackjack, print_hand};
use crate::menu::{MenuChoice, PostGameChoice, display_help, get_user_choice};
use crate::game_action::PlayerActionResult;
use crate::blackjack::{player_turn, dealer_turn};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn run_game(cfg: &GameConfig) -> anyhow::Result<()> {
    println!("現在の残高: {}{}", cfg.player_starting_bank, cfg.currency_name);
    println!("ベット額: {}{}", cfg.bet_amount, cfg.currency_name);
    println!();

    let mut deck = build_deck();
    shuffle_deck(&mut deck);

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

    // Check for dealer blackjack
    let dealer_has_blackjack = is_blackjack(&dealer_hand);
    if dealer_has_blackjack {
        println!("\nディーラーがブラックジャック！");
        print_hand("ディーラー", &dealer_hand, false);
        
        // Check if player also has blackjack (only applies to first hand before any splits)
        let player_has_blackjack = player_hands.len() == 1 && is_blackjack(&player_hands[0]);
        
        if player_has_blackjack {
            println!("プレイヤーもブラックジャック！引き分けです");
            print_hand("プレイヤー", &player_hands[0], false);
            println!("結果: 引き分け - 変動なし");
        } else {
            println!("ディーラーの勝利");
            print_hand("プレイヤー", &player_hands[0], false);
            println!("損失: -{}{}", cfg.bet_amount, cfg.currency_name);
        }
        
        let continue_playing = show_post_game_menu(cfg)?;
        if continue_playing {
            return run_game(cfg);
        }
        return Ok(());
    }

    let mut total_bet = cfg.bet_amount;

    // Process each player hand (initially just one, but can become multiple with splits)
    let mut hand_index = 0;
    while hand_index < player_hands.len() {
        let mut current_hand = player_hands[hand_index].clone();
        
        println!("\n--- 手札 {} ---", hand_index + 1);
        
        // Check for player blackjack
        if is_blackjack(&current_hand) {
            println!("ブラックジャック！");
            print_hand("プレイヤー", &current_hand, false);
            player_hands[hand_index] = current_hand;
            hand_index += 1; // Move to next hand
            continue;
        }
        
        // Check if this is the first action for this hand (for split/double/surrender eligibility)
        let is_first_action = current_hand.len() == 2;
        
        match player_turn(&mut deck, &mut current_hand, is_first_action)? {
            PlayerActionResult::Stand => {
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

    // Check if all player hands are bust - if so, skip dealer turn
    let all_hands_bust = player_hands.iter().all(|hand| hand_value(hand) > 21);

    if all_hands_bust {
        println!("\n--- 全ての手札がバスト ---");
        print_hand("ディーラー", &dealer_hand, false);
        println!("ディーラーはカードを引く必要がありません");
    } else {
        // dealer reveals and plays
        println!("\n--- ディーラーのターン ---");
        print_hand("ディーラー", &dealer_hand, false);
        dealer_turn(&mut deck, &mut dealer_hand);
        print_hand("ディーラー最終", &dealer_hand, false);
    }

    let dv = hand_value(&dealer_hand);

    // Evaluate each hand
    let mut total_winnings = 0i32;
    
    for (i, hand) in player_hands.iter().enumerate() {
        let pv = hand_value(hand);
        println!("\n--- 手札 {} の結果 ---", i + 1);
        print_hand(&format!("プレイヤー手札{}", i + 1), hand, false);
        
        let player_has_bj = is_blackjack(hand);
        let dealer_has_bj = is_blackjack(&dealer_hand);
        
        let result = if pv > 21 {
            "バスト - 負け"
        } else if dv > 21 {
            "ディーラーがバスト - 勝ち"
        } else if player_has_bj && !dealer_has_bj {
            "ブラックジャック - 勝ち"
        } else if !player_has_bj && dealer_has_bj {
            "ディーラーブラックジャック - 負け"
        } else if player_has_bj && dealer_has_bj {
            "両方ブラックジャック - 引き分け"
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
            .items(PostGameChoice::menu_items())
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
