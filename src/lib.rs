use std::collections::{HashMap, HashSet};

const ACE_VALUE: u8 = 14;
const KING_VALUE: u8 = 13;
const QUEEN_VALUE: u8 = 12;
const JACK_VALUE: u8 = 11;

#[derive(Eq, Hash, PartialEq)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

struct Card {
    value: u8,
    suit: Suit,
}

enum HandType {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

struct Hand<'a> {
    tie_breaker: Vec<u8>,
    input_string: &'a str,
    hand_type: HandType,
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let parsed_hands = hands.iter().map(|hand| parse_hand(hand));

    unimplemented!("Out of {:?}, which hand wins?", hands)
}

fn parse_hand<'a>(hand_str: &'a str) -> Hand {
    // parse cards
    let mut cards: Vec<_> = hand_str
        .clone()
        .split_whitespace()
        .map(|card| parse_card(card))
        .collect();

    // characterise hand
    // - count cards of a kind
    // - identify straights and flushes

    cards.sort_by_key(|c| c.value);
    let mut prev_value: u8 = 1;

    let mut suits: HashSet<Suit> = HashSet::new();
    let mut of_a_kinds: HashMap<u8, u8> = HashMap::new();
    let mut is_straight = true;

    for card in cards {
        // flush
        suits.insert(card.suit);

        // pairs/of a kind
        if prev_value == card.value {
            of_a_kinds
                .entry(card.value)
                .and_modify(|v| *v += 1)
                .or_insert(2);
        }

        // straights
        // - check consecutive values
        if prev_value + 1 != card.value {
            // - check for low ace
            if prev_value != 5 && card.value != ACE_VALUE {
                is_straight = false;
            }
        }

        prev_value = card.value;
    }

    // determine hand type
    // - use characteristics from above to determine hand type
    let mut tie_breaker = Vec::new();
    let hand_type = HandType::Flush;

    return Hand {
        tie_breaker: tie_breaker,
        input_string: hand_str,
        hand_type: hand_type,
    };
}

fn parse_card(card_str: &str) -> Card {
    let suit = match card_str.get(1..2) {
        Some("C") => Suit::Club,
        Some("S") => Suit::Spade,
        Some("H") => Suit::Heart,
        Some("D") => Suit::Diamond,
        None => panic!("Malformed card string: {:?}", card_str),
        Some(_) => panic!("Malformed card string: {:?}", card_str),
    };

    let value_str = card_str.get(0..1).unwrap();

    let value = match value_str.parse::<u8>() {
        Ok(v) => v,
        Err(_) => match value_str {
            "A" => ACE_VALUE,
            "K" => KING_VALUE,
            "Q" => QUEEN_VALUE,
            "J" => JACK_VALUE,
            _ => panic!("Malformed card string: {:?}", card_str),
        },
    };

    return Card {
        suit: suit,
        value: value,
    };
}
