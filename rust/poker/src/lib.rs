use cards::parse_hand;
use game::ClassifiedHand;

mod cards;
mod game;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let classified_hands = hands
        .iter()
        .copied()
        .map(|h| parse_hand(h).map(|hand| (h, ClassifiedHand::new(hand))))
        .collect::<Result<Vec<_>, _>>();

    match classified_hands {
        Ok(mut hs) => {
            hs.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            // Top has the highest order and is one of
            // the winning hands.
            let top = hs[0];

            // Collect any other wining hands and map
            // back to the &str hands passed in.
            hs.into_iter()
                .take_while(|c| top.1.cmp(&c.1).is_eq())
                .map(|c| c.0)
                .collect()
        }
        Err(err) => {
            eprintln!("Failed to parse hands {err:?}");
            vec![]
        }
    }
}
