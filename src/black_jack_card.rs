use std::fmt::{Display, Formatter, Result};

#[derive(Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Display for Suit {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

pub struct BlackJackCard {
    pub suit: Suit,
    pub value: u8,
    pub name: String,
    pub is_face_up: bool,
}

// make struct printable
impl Display for BlackJackCard {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{} {} {} of {}",
            (if self.is_face_up {"An"} else {"A"}),
            (if self.is_face_up {"up"} else {"down"}),
            self.name,
            self.suit
        )
    }
}

impl BlackJackCard {
    pub fn new(suit: Suit, value: u8, name: String, is_face_up: bool) -> BlackJackCard {
        BlackJackCard {
            suit,
            value,
            is_face_up,
            name,
        }
    }
}