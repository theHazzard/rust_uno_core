#[derive(Debug)]
pub enum CardValue {
    Number(u8),
    None,
}

#[derive(Debug)]
pub enum CardType {
    Number,
    Plus2,
    Plus4,
    SwitchColor,
    Turn,
    Block,
}

#[derive(Debug)]
pub enum CardColor {
    Red,
    Blue,
    Yellow,
    Green,
    Black,
    None,
}

#[derive(Debug)]
pub struct Card {
    color: CardColor,
    value: CardValue,
    pub cardType: CardType,
}

impl Card {
    pub fn new(cardType: CardType, value: CardValue, color: CardColor) -> Self {
        Card {
            cardType,
            value,
            color,
        }
    }
}

