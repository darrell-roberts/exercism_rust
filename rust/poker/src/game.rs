use crate::cards::{Hand, Rank};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ClassifiedHand {
    hand: Hand,
    hand_type: HandType,
}

impl ClassifiedHand {
    pub fn new(hand: Hand) -> Self {
        Self {
            hand_type: classify_hand(hand),
            hand,
        }
    }
}

impl Ord for ClassifiedHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.hand_type.cmp(&other.hand_type).then_with(|| {
            let mut sorted_ranks = self.hand.map(|h| h.rank);
            sorted_ranks.sort_unstable_by(|a, b| b.cmp(a));

            let mut other_sorted_ranks = other.hand.map(|h| h.rank);
            other_sorted_ranks.sort_unstable_by(|a, b| b.cmp(a));

            let sequence_sort = || sorted_ranks.cmp(&other_sorted_ranks);

            match self.hand_type {
                HandType::FourOfAKind | HandType::FullHouse => {
                    group_sort(&self.hand).cmp(group_sort(&other.hand))
                }
                HandType::Straight | HandType::StraightFlush => {
                    let low_ace_1 = sorted_ranks[0] == Rank::Ace;
                    let low_ace_2 = other_sorted_ranks[0] == Rank::Ace;

                    match (low_ace_1, low_ace_2) {
                        (true, true) => std::cmp::Ordering::Equal,
                        (true, false) => sorted_ranks[1].cmp(&other_sorted_ranks[0]),
                        (false, true) => sorted_ranks[0].cmp(&other_sorted_ranks[1]),
                        (false, false) => sequence_sort(),
                    }
                }
                _ => sequence_sort(),
            }
        })
    }
}

impl PartialOrd for ClassifiedHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn is_flush(hand: &Hand) -> Option<HandType> {
    let suit = hand[0].suit;
    hand.iter()
        .all(|c| c.suit == suit)
        .then_some(HandType::Flush)
}

fn is_straight(hand: &mut Hand) -> Option<HandType> {
    use Rank::*;
    hand.sort_unstable_by_key(|c| c.rank);

    (hand.map(|c| c.rank) == [Two, Three, Four, Five, Ace]
        || hand
            .iter()
            .map(|c| c.rank as i8 - hand[0].rank as i8)
            .eq(0..5))
    .then_some(HandType::Straight)
}

fn rank_counts(hand: &Hand) -> HashMap<Rank, usize> {
    let mut rank_counts = HashMap::new();
    for c in hand {
        rank_counts
            .entry(c.rank)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    rank_counts
}

fn group_sort(hand: &Hand) -> impl Iterator<Item = Rank> {
    let mut rank_counts = rank_counts(hand).into_iter().collect::<Vec<_>>();
    rank_counts.sort_unstable_by(|rc1, rc2| rc2.1.cmp(&rc1.1));
    rank_counts.into_iter().map(|rc| rc.0)
}

fn by_rank_groups(hand: &Hand) -> Option<HandType> {
    let rank_counts = rank_counts(hand);

    let groups = rank_counts.len();
    let mut counts = rank_counts.into_values().collect::<Vec<_>>();
    counts.sort_unstable_by(|a, b| b.cmp(a));

    match groups {
        2 if counts[0] == 3 => Some(HandType::FullHouse),
        2 if counts[0] == 4 => Some(HandType::FourOfAKind),
        3 if counts[0] == 3 => Some(HandType::ThreeOfAKind),
        3 if counts[0] == 2 => Some(HandType::TwoPair),
        4 if counts[0] == 2 => Some(HandType::OnePair),
        _ => None,
    }
}

fn classify_hand(mut hand: Hand) -> HandType {
    let flush = is_flush(&hand);
    let straight = is_straight(&mut hand);
    match (flush, straight) {
        (Some(_), Some(_)) => HandType::StraightFlush,
        (None, Some(straight)) => straight,
        (Some(flush), None) => flush,
        (None, None) => by_rank_groups(&hand).unwrap_or(HandType::HighCard),
    }
}
