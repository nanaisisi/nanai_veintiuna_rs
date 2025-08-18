use crate::card::{Card, hand_value, print_hand};
use crate::game_action::{GameAction, PlayerActionResult, can_double_down, can_split, can_surrender};
use dialoguer::{Select, theme::ColorfulTheme};

pub fn player_turn(deck: &mut Vec<Card>, hand: &mut Vec<Card>, is_first_turn: bool) -> anyhow::Result<PlayerActionResult> {
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

pub fn dealer_turn(deck: &mut Vec<Card>, hand: &mut Vec<Card>) {
    while hand_value(hand) < 17 {
        if let Some(card) = deck.pop() {
            hand.push(card);
        } else {
            break;
        }
    }
}
