use crate::black_jack_card::{BlackJackCard, Suit};
extern crate rand;
use rand::seq::SliceRandom;

pub struct BlackJackDeck {
	cards: Vec<BlackJackCard>,
}

// make BlackJackDeck printable
impl std::fmt::Display for BlackJackDeck {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let mut cards = String::new();
		for card in &self.cards {
			cards.push_str(&format!("{}, ", card));
		}
		write!(f, "{}with a sum of {}", cards, self.get_cards_sum())
	}
}

impl BlackJackDeck {
	pub fn new(is_full_and_shuffled: bool) -> BlackJackDeck {
		if is_full_and_shuffled {
			BlackJackDeck {
				cards: BlackJackDeck::get_full_and_shuffled_deck(),
			}
		} else {
			BlackJackDeck { cards: vec![] }
		}
	}
	
	pub fn get_full_and_shuffled_deck() -> Vec<BlackJackCard> {
		let mut cards: Vec<BlackJackCard> = vec![];
		for suit in vec![Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
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
				let cloned_suit = suit.clone();
				let card = BlackJackCard::new(cloned_suit, value, name, false);
				cards.push(card);
			}
		}
		cards.shuffle(&mut rand::thread_rng());
		
		cards
	}
	
	pub fn add_card(&mut self, card: BlackJackCard) {
		self.cards.push(card);
	}
	
	pub fn deal_card(&mut self) -> BlackJackCard {
		self.cards.pop() // returns Option<BlackJackCard>
			.expect("No more cards in deck") // returns BlackJackCard;
	}
	
	pub fn get_cards_sum(&self) -> u8 {
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