#[derive(Clone)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
    pub name_starts_with_vowel: bool,
}

// make struct printable
impl std::fmt::Display for BlackJackCard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} {} of {}",
            (if self.name_starts_with_vowel {"An"} else {"A"}),
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
            name_starts_with_vowel: ["Ace", "8"].contains(&name.as_str()),
            name,
        }
    }
}