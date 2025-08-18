use rand::seq::SliceRandom;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
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
pub struct Card(pub Rank);

impl Card {
    pub fn value(&self) -> u8 {
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
    
    pub fn is_ace(&self) -> bool {
        matches!(self.0, Rank::Ace)
    }
    
    pub fn short(&self) -> &'static str {
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

pub fn build_deck() -> Vec<Card> {
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

pub fn shuffle_deck(deck: &mut Vec<Card>) {
    let mut rng = rand::rng();
    deck.shuffle(&mut rng);
}

pub fn hand_value(hand: &[Card]) -> u8 {
    let mut sum: u8 = hand.iter().map(|c| c.value()).sum();
    let mut aces = hand.iter().filter(|c| c.is_ace()).count();
    while sum > 21 && aces > 0 {
        sum -= 10; // count ace as 1 instead of 11
        aces -= 1;
    }
    sum
}

pub fn is_blackjack(hand: &[Card]) -> bool {
    hand.len() == 2 && hand_value(hand) == 21
}

pub fn print_hand(name: &str, hand: &[Card], hide_first: bool) {
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
