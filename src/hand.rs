use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

use super::*;

/// A `Hand` is zero or more cards that represents some aspect of a game,
/// e.g. the cards a person is holding. A hand may be shuffled or sorted
/// and there are functions for adding or removing cards. Unlike a `Deck`,
/// there is no concept of dealt or undealt cards.
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = String::new();
        for (i, card) in self.cards.iter().enumerate() {
            result.push_str(&card.to_str());
            if i < self.cards.len() - 1 {
                result.push(',');
            }
        }
        write!(f, "{}", result)
    }
}

impl Clone for Hand {
	fn clone(&self) -> Hand {
		return Hand {
			cards: self.cards.clone()
		}
	}
}

impl<'a> AddAssign<&'a Hand> for Hand {
    fn add_assign(&mut self, rhs: &Hand) {
        self.push_hand(rhs);
    }
}

impl AddAssign<Card> for Hand {
    fn add_assign(&mut self, rhs: Card) {
        self.push_card(rhs);
    }
}

impl Cards for Hand {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }
}

impl Hand {
    /// Make a new empty `Hand`
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    /// Makes a `Hand` from an existing hand
    pub fn from_hand(hand: &Hand) -> Hand {
        Hand::from_cards(hand.cards())
    }

    /// Makes a `Hand` from a slice
    pub fn from_cards(cards: &[Card]) -> Hand {
        Hand { cards: Vec::from(cards) }
    }

    /// Constructs a `Hand` from a slice of strings with abbreviated card rank / suit values
    pub fn from_strings(card_slice: &[&str]) -> Hand {
        let mut cards: Vec<Card> = Vec::with_capacity(card_slice.len());
        for s in card_slice {
            let card = card!(s);
            cards.push(card);
        }
        Hand { cards: cards }
    }

    /// Adds one `Card` to the `Hand`
    pub fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Adds zero or more cards to the `Hand`
    pub fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Adds zero or more cards from some other `Hand`
    pub fn push_hand(&mut self, other: &Hand) {
        self.cards.extend(other.cards());
    }

    /// Returns the number of cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    
    /// Clears the `Hand` (makes it empty)
    pub fn clear(&mut self) {
    	self.cards.clear();
    }
    
    /// Removes a `Card` from the `Hand` and returns it, panics if index does not exist
    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the `Hand`
    pub fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            self.remove_card(*c);
        }
    }

    /// Removes first instance of the matching card from the `Hand`
    pub fn remove_card(&mut self, card: Card) {
        let found = self.cards.iter().position(|c| *c == card);
        if found.is_some() {
            self.cards.remove(found.unwrap());
        }
    }

    /// Returns cards of the specified `Rank`
    pub fn cards_of_rank(&self, rank: Rank) -> Vec<Card> {
        cards_of_rank(&self.cards, rank)
    }

    /// Returns cards of the specified `Suit`
    pub fn cards_of_suit(&self, suit: Suit) -> Vec<Card> {
        cards_of_suit(&self.cards, suit)
    }
}
