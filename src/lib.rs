use std::collections::{HashMap, HashSet};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

const ACE_VALUE: u8 = 14;
const KING_VALUE: u8 = 13;
const QUEEN_VALUE: u8 = 12;
const JACK_VALUE: u8 = 11;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
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

#[derive(PartialEq, EnumIter)]
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
    let mut parsed_hands: Vec<Hand> = hands.iter().map(|hand| parse_hand(hand)).collect();

    for hand_type in HandType::iter() {
        if parsed_hands.iter().any(|h| h.hand_type == hand_type) {
            let winning_hands: Vec<&Hand> = parsed_hands.iter().filter(|h| h.hand_type == hand_type).collect();
            let returned_hands: Vec<&'a str> = winning_hands.iter().map(|h| h.input_string).collect();
            return Some(returned_hands);
        }
    }
    return None;
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

    for card in cards.iter() {
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

    let max_of_a_kind_count = of_a_kinds.values().max().unwrap_or(&0);

    // determine hand type
    // - use characteristics from above to determine hand type
    let mut tie_breaker = Vec::new();
    let card_values: Vec<u8> = cards.iter().rev().map(|c| c.value).collect();
    let hand_type: HandType;

    if is_straight && suits.len() == 1 {
        // straight flush
        hand_type = HandType::StraightFlush;

        // TODO same tie_breaker code for straight and straight flush
        let mut highest_card = &cards.first().unwrap().value;

        // handle low ace
        if *highest_card == ACE_VALUE {
            let second_card = &cards[1].value;
            if *second_card == 5 {
                highest_card = second_card;
            }
        }

        tie_breaker.push(*highest_card);
    } else if *max_of_a_kind_count == 4 {
        // 4 of a kind
        hand_type = HandType::FourOfAKind;

        // TODO same tie_breaker code for 4, 3, 2 "of a kind"
        let value = of_a_kinds.keys().next().unwrap();
        tie_breaker.push(*value);
        tie_breaker.extend(card_values.iter());
    } else if *max_of_a_kind_count == 3 && of_a_kinds.len() == 2 {
        // full house
        hand_type = HandType::FullHouse;

        let mut pair_value = 0;
        let mut triple_value = 0;

        for (value, count) in of_a_kinds {
            if count == 3 {
                triple_value = value;
            }
            if count == 2 {
                pair_value = value;
            }
        }
        tie_breaker.push(triple_value);
        tie_breaker.push(pair_value);
    } else if suits.len() == 1 {
        // flush
        hand_type = HandType::Flush;
        tie_breaker.extend(card_values.iter());
    } else if is_straight {
        // straight
        hand_type = HandType::Straight;

        let mut highest_card = &cards.first().unwrap().value;

        // handle low ace
        if *highest_card == ACE_VALUE {
            let second_card = &cards[1].value;
            if *second_card == 5 {
                highest_card = second_card;
            }
        }

        tie_breaker.push(*highest_card);
    } else if *max_of_a_kind_count == 3 {
        // three of a kind
        hand_type = HandType::ThreeOfAKind;

        // only 1 of_a_kinds, otherwise it would be a full house
        let triple_value = of_a_kinds.keys().next().unwrap();
        tie_breaker.push(*triple_value);
        tie_breaker.extend(card_values.iter());
    } else if of_a_kinds.len() == 2 {
        // two pair
        hand_type = HandType::TwoPair;
    } else if of_a_kinds.len() == 1 {
        // one pair
        hand_type = HandType::OnePair;

        let pair_value = of_a_kinds.keys().next().unwrap();
        tie_breaker.push(*pair_value);
        tie_breaker.extend(card_values.iter());
    } else {
        // high card
        hand_type = HandType::HighCard;

        tie_breaker.extend(card_values.iter());
    }

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
