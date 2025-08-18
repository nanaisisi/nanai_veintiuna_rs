use crate::card::Card;

#[derive(Debug, Clone)]
pub enum GameAction {
    Hit,
    Stand,
    DoubleDown,
    Split,
    Surrender,
}

#[derive(Debug, Clone)]
pub enum PlayerActionResult {
    Continue,
    Stand,
    DoubleDown,
    Split(Vec<Card>, Vec<Card>), // Two split hands
    Surrender,
}

impl GameAction {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(GameAction::Hit),
            1 => Some(GameAction::Stand),
            2 => Some(GameAction::DoubleDown),
            3 => Some(GameAction::Split),
            4 => Some(GameAction::Surrender),
            _ => None,
        }
    }

    pub fn menu_items(can_double: bool, can_split: bool, can_surrender: bool) -> Vec<&'static str> {
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

    pub fn get_valid_actions(can_double: bool, can_split: bool, can_surrender: bool) -> Vec<Self> {
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

pub fn can_double_down(hand: &[Card]) -> bool {
    hand.len() == 2
}

pub fn can_split(hand: &[Card]) -> bool {
    hand.len() == 2 && hand[0].value() == hand[1].value()
}

pub fn can_surrender(hand: &[Card]) -> bool {
    hand.len() == 2
}
