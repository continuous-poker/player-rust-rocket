use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Suit {
    HEARTS,
    SPADES,
    CLUBS,
    DIAMONDS
}