mod black_jack_card;
mod black_jack_deck;

// just a test for now, will translate the rest of the C# code later
fn main() {
	let mut deck = black_jack_deck::BlackJackDeck::new(true);
	let mut player1_hand = black_jack_deck::BlackJackDeck::new(false);
	let mut player2_hand = black_jack_deck::BlackJackDeck::new(false);
	
	// deal 2 cards to each player
	for _ in 0..2 {
		player1_hand.add_card(deck.deal_card());
		player2_hand.add_card(deck.deal_card());
	}
	
	println!("Player 1 hand: {}", player1_hand);
	println!("Player 2 hand: {}", player2_hand);
}
