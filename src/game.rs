use crate::config::GameConfig;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io::{self, Write};

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
        print_hand("Player", hand, false);
        if hand_value(hand) >= 21 {
            break;
        }
        print!("Hit or Stand? (h/s) ");
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
            _ => println!("Type 'h' or 's'"),
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
    // simple single-hand session demonstrating a player-advantaged rule set
    let mut deck = build_deck();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    let mut player_hand = Vec::new();
    let mut dealer_hand = Vec::new();

    // initial deal: player, dealer, player, dealer
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());
    player_hand.push(deck.pop().unwrap());
    dealer_hand.push(deck.pop().unwrap());

    print_hand("Dealer", &dealer_hand, true);

    // simple player strategy biased toward winning: stand on soft 18+
    player_turn(&mut deck, &mut player_hand);

    // dealer reveals and plays
    print_hand("Dealer", &dealer_hand, false);
    dealer_turn(&mut deck, &mut dealer_hand);
    print_hand("Dealer final", &dealer_hand, false);

    let pv = hand_value(&player_hand);
    let dv = hand_value(&dealer_hand);

    // apply a simple player edge: if player within small margin, boost as win
    let result = if pv > 21 {
        "Player busts - lose"
    } else if dv > 21 {
        "Dealer busts - player wins"
    } else if pv > dv {
        "Player wins"
    } else if pv < dv {
        "Player loses"
    } else {
        // tie broken by player_edge bias
        if cfg.player_edge > 0.0 {
            "Player wins (edge)"
        } else {
            "Push"
        }
    };

    println!("Player: {} vs Dealer: {} => {}", pv, dv, result);
    Ok(())
}
