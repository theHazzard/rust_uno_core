mod card;
pub use card::*;

use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
  pub fn new() -> Self {
    let mut deck = Vec::new();

    // Reds
    for n in 0..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Red))
    }
    for n in 1..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Red))
    }
    // Red Turn, +2, Block
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Red));
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Red));

    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Red));
    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Red));

    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Red));
    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Red));

    // Blues
    for n in 0..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Blue))
    }
    for n in 1..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Blue))
    }
    // Blue Turn, +2, Block
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Blue));
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Blue));

    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Blue));
    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Blue));

    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Blue));
    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Blue));

    // Yellows
    for n in 0..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Yellow))
    }
    for n in 1..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Yellow))
    }

    // Blue Turn, +2, Block
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Yellow));
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Yellow));

    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Yellow));
    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Yellow));

    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Yellow));
    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Yellow));

    // Greens
    for n in 0..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Green))
    }
    for n in 1..10 {
      deck.push(Card::new(CardType::Number, CardValue::Number(n), CardColor::Green))
    }

    // Green Turn, +2, Block
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Green));
    deck.push(Card::new(CardType::Plus2, CardValue::None, CardColor::Green));

    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Green));
    deck.push(Card::new(CardType::Turn, CardValue::None, CardColor::Green));

    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Green));
    deck.push(Card::new(CardType::Block, CardValue::None, CardColor::Green));

    //Black cards
    for _ in 0..4 {
      deck.push(Card::new(CardType::Plus4, CardValue::None, CardColor::Black));
      deck.push(Card::new(CardType::SwitchColor, CardValue::None, CardColor::Black));  
    }

    for _ in 0..3 {
      deck.shuffle(&mut thread_rng());
    }

    Deck {
      cards: deck,
    }
  }

  pub fn draw(&mut self) -> Card {
    self.cards.pop().unwrap()
  }

  pub fn refill(&mut self, cards: &mut Vec<Card>) {
    self.cards.append(cards);
    for _ in 0..3 {
      self.cards.shuffle(&mut thread_rng());
    }
  }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn should_have_enough_cards() {
      let deck = Deck::new();
      print!("{:?}", deck);
      assert!(deck.cards.len() == 108);
    }
}