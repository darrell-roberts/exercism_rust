#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
    Club,
    Diamond,
    Spade,
    Heart,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// A Poker hand has 5 cards.
pub type Hand = [Card; 5];

#[derive(Debug, Copy, Clone)]
pub enum InvalidHand {
    UnknownRank,
    UnknownSuit,
    BadInput,
    BadLength,
}

impl TryFrom<&str> for Rank {
    type Error = InvalidHand;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Rank::*;
        Ok(match value {
            "A" => Ace,
            "2" => Two,
            "3" => Three,
            "4" => Four,
            "5" => Five,
            "6" => Six,
            "7" => Seven,
            "8" => Eight,
            "9" => Nine,
            "10" => Ten,
            "J" => Jack,
            "Q" => Queen,
            "K" => King,
            _ => return Err(InvalidHand::UnknownRank),
        })
    }
}

impl TryFrom<char> for Suit {
    type Error = InvalidHand;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Suit::*;
        Ok(match value {
            'C' => Club,
            'D' => Diamond,
            'S' => Spade,
            'H' => Heart,
            _ => return Err(InvalidHand::UnknownSuit),
        })
    }
}

impl TryFrom<&str> for Card {
    type Error = InvalidHand;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let suit = value
            .chars()
            .last()
            .ok_or(InvalidHand::BadInput)
            .and_then(Suit::try_from)?;

        let rank = Rank::try_from(&value[..value.len() - 1])?;

        Ok(Card { rank, suit })
    }
}

pub fn parse_hand(hand: &str) -> Result<Hand, InvalidHand> {
    hand.split(' ')
        .map(Card::try_from)
        .collect::<Result<Vec<_>, _>>()
        .and_then(|hand| hand.try_into().map_err(|_| InvalidHand::BadLength))
}

#[cfg(test)]
mod test {
    use super::{parse_hand, Card, Rank::*, Suit::*};
    use std::assert_eq;

    #[test]
    fn test_parse_hand() {
        assert_eq!(
            parse_hand("4D 5S 6S 8D 3C").unwrap(),
            [
                Card {
                    rank: Four,
                    suit: Diamond
                },
                Card {
                    rank: Five,
                    suit: Spade
                },
                Card {
                    rank: Six,
                    suit: Spade,
                },
                Card {
                    rank: Eight,
                    suit: Diamond
                },
                Card {
                    rank: Three,
                    suit: Club
                }
            ]
        )
    }

    #[test]
    fn test_ord() {
        assert!(Five > Three)
    }
}
