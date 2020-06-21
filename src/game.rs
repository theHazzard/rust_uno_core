mod deck;
mod player;
pub use deck::*;
pub use player::*;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct Game {
  pub deck: Deck,
  pub played: Vec<Card>,
  pub players: Vec<Player>,
  pub turn: u8,
  pub direction: Direction,
  pub currentColor: CardColor,
}

impl Game {
  pub fn new(players: Vec<Player>) -> Self {
    Game {
      deck: Deck::new(),
      played: Vec::new(),
      players,
      turn: 0,
      direction: Direction::Forward,
      currentColor: CardColor::None,
    }
  }

  pub fn start(&mut self) {
    // drawn card for every player
    for _ in 0..7 {
      for player in &mut self.players {
        player.receive_card(self.deck.draw());
      }
    }

    self.played.splice(0..0, vec![self.deck.draw()]);
  }

  pub fn play(&mut self, playerNumber: usize, cardNumber: usize, chosenColor: CardColor) {
    // check if there's enough cards left on the 
    // deck to fill requirements
    // nah, for now, if there is less than 7 cards, refill
    if self.deck.cards.len() < 7 {
      let first_card = self.played.remove(0);
      self.deck.refill(&mut self.played);
      self.played = vec![first_card];
    }
    
    // get the card from the player
    let card = self.players[playerNumber].play_card(cardNumber);

    // play the card effect
    // Number, ok
    // Plus2, ok
    // Plus4, ok 
    // SwitchColor, ok
    // Turn, ok
    // Block, ok
    match card.cardType {
        CardType::Number => {
        },
        CardType::Turn => {
          match self.direction {
              Direction::Forward => self.direction = Direction::Backward,
              Direction::Backward => self.direction = Direction::Forward,
          }
        },
        CardType::Plus2 => {
          let next_player = self.get_next_player(playerNumber as u8);

          for _ in 0..2 {
            self.players[next_player as usize].receive_card(self.deck.draw());
          }
        },
        CardType::Plus4 => {
          let next_player = self.get_next_player(playerNumber as u8);
          self.currentColor = chosenColor;
          for _ in 0..4 {
            self.players[next_player as usize].receive_card(self.deck.draw());
          }
        },
        CardType::SwitchColor => {
          self.currentColor = chosenColor;
        }
        CardType::Block => {
          // simulate wasted turn
          self.do_turn();
        },
    }

    // move turn
    self.do_turn();

    //play the card
    self.played.splice(0..0, vec![card]);
  }

  fn do_turn(&mut self) {
    match self.direction {
      Direction::Forward => {
        if self.turn + 1 == self.players.len() as u8 {
          self.turn = 0;
        } else {
          self.turn += 1;
        }
      },
      Direction::Backward => {
        if self.turn == 0 {
          self.turn = self.players.len() as u8;
        } else {
          self.turn -= 1;
        }
      },
    }
  }

  fn get_next_player(&self, playerNumber: u8) -> u8 {
    match self.direction {
      Direction::Forward => {
        if playerNumber == self.players.len() as u8 {
          0
        } else {
          playerNumber + 1
        }
      },
      Direction::Backward => {
        if playerNumber == 0 {
          self.players.len() as u8
        } else {
          playerNumber- 1
        }
      },
    }
  }
}

#[cfg(test)]
mod test_super {
    use super::*;
}