use crate::game::deck::*;
use uuid::Uuid;


#[derive(Debug)]
pub struct Player {
    id: Uuid,
    name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Self {
      Player {
        id: Uuid::new_v4(),
        name: String::from("test user"),
        hand: Vec::new(),
      }
    }

    pub fn receive_card(&mut self, card: Card) {
      self.hand.push(card);
    }

    pub fn play_card(&mut self, card_position: usize) -> Card {
      self.hand.remove(card_position)
    }
}