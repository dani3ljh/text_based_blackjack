mod black_jack_card;
use black_jack_card::BlackJackCard;

mod black_jack_deck;
use black_jack_deck::BlackJackDeck;

mod get_input;
use get_input::string_with_values;

fn main() {
	const NUMBER_OF_PLAYERS: i32 = 2;

	let mut deck = BlackJackDeck::new();
	deck.fill_deck();
	deck.shuffle();

	let mut players: Vec<BlackJackDeck> = vec![];

	for _ in 0..NUMBER_OF_PLAYERS {
		players.push(BlackJackDeck::new());
	}
	
	// deal 2 cards to each player
	for player in &mut players {
		player.push(deck.pop());
	}
	for player in &mut players {
		let mut card = deck.pop();
		card.is_face_up = true;
		player.push(card);
	}
	
	// println!("Player {:?} wins", play_turn(&mut players, &mut deck).iter().map(|x| x + 1).to_vec());
	let players_won = play_turn(&mut players, &mut deck);
	print!("Players ");
	for player_index in players_won.iter().map(|x| x + 1) {
		print!("{}, ", player_index);
	}
	println!(" won");
}

fn play_turn(players: &mut Vec<BlackJackDeck>, deck: &mut BlackJackDeck) -> Vec<usize> {
	let mut max_value: u8 = 0;
	let mut res: Vec<usize> = vec![];

	for (i, player) in players.iter_mut().enumerate() {
		let player_num = i + 1;
		loop {
			println!("Player {player_num}'s hand: {player}");

			let hand_value = player.get_sum();

			if hand_value > 21 {
				println!("Player {player_num} busts");
				break;
			}

			if max_value < hand_value {
				max_value = hand_value;
				res = vec![i];
			} else if max_value == hand_value {
				res.push(i);
			}
			
			if hand_value == 21 {
				break;
			}
			
			match string_with_values(vec!["hit".to_string(), "stand".to_string()]).as_str() {
				"hit" => {
					let mut card = deck.pop();
					card.is_face_up = true;
					player.push(card);
				},
				"stand" => break,
				&_ => todo!(),
			}
		}
	}

	res
}