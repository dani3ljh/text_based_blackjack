use crate::black_jack_card::{BlackJackCard, Suit};
extern crate rand;
use rand::seq::SliceRandom;
use std::fmt::{Display, Formatter, Result};

pub struct BlackJackDeck {
	cards: Vec<BlackJackCard>,
}

// make BlackJackDeck printable
impl Display for BlackJackDeck {
	fn fmt(&self, f: &mut Formatter) -> Result {
		for card in &self.cards {
			if let Err(e) = write!(f, "{}, ", card) {
				return Err(e);
			}
		}
		write!(f, "with a sum of {}", self.get_sum())
	}
}

impl BlackJackDeck {
	pub fn new() -> BlackJackDeck {
		BlackJackDeck { cards: vec![] }
	}

	pub fn fill_deck(&mut self) {
		let mut cards: Vec<BlackJackCard> = vec![];

		for suit in [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
			for i in 1..14 {
				let name = match i {
					1 => "Ace".to_string(),
					2..=10 => i.to_string(),
					11 => "Jack".to_string(),
					12 => "Queen".to_string(),
					13 => "King".to_string(),
					_ => unreachable!("Invalid value")
				};

				let value = match i {
					1 => 11,
					2..=10 => i,
					11..=13 => 10,
					_ => unreachable!("Invalid value")
				};

				let card = BlackJackCard::new(suit.clone(), value, name, false);

				cards.push(card);
			}
		}

		self.cards = cards;
	}
	
	pub fn shuffle(&mut self) {
		self.cards.shuffle(&mut rand::thread_rng());
	}
	
	pub fn push(&mut self, card: BlackJackCard) {
		self.cards.push(card);
	}
	
	pub fn pop(&mut self) -> BlackJackCard {
		self.cards.pop().expect("No more cards in deck")
	}
	
	pub fn get_sum(&self) -> u8 {
		let mut sum = 0;
		let mut aces_count = 0;
		for card in &self.cards {
			sum += card.value;
			if card.value == 11 {
				aces_count += 1;
			}
		}
		while sum > 21 && aces_count > 0 {
			sum -= 10;
			aces_count -= 1;
		}
		sum
	}
}