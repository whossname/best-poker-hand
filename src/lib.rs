use std::collections::{HashMap, HashSet};

const ACE_VALUE: u8 = 14;
const KING_VALUE: u8 = 13;
const QUEEN_VALUE: u8 = 12;
const JACK_VALUE: u8 = 11;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
enum Suit {
    Spade,
    Club,
    Heart,
    Diamond,
}

#[derive(Debug)]
struct Card {
    value: u8,
    suit: Suit,
}

#[derive(Ord, Eq, PartialEq, PartialOrd, Clone, Copy, Debug)]
enum HandType {
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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Hand<'a> {
    hand_type: HandType,
    tie_breaker: Vec<u8>,
    input_string: &'a str,
}

type Suits = HashSet<Suit>;
type OfAKinds = HashMap<u8, u8>;
type IsStraight = bool;
type HandProfile = (Suits, OfAKinds, IsStraight);

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut parsed_hands: Vec<Hand> = hands.iter().map(|hand| parse_hand(hand)).collect();
    parsed_hands.sort();
    parsed_hands.reverse();

    let (winning_hand_type, winning_tie_breaker) = match parsed_hands.first() {
        None => return None,
        Some(x) => (x.hand_type.clone(), x.tie_breaker.clone()),
    };

    let winners: Vec<Hand> = parsed_hands
        .drain(..)
        .take_while(|h| winning_hand_type == h.hand_type && winning_tie_breaker == h.tie_breaker)
        .collect();

    let winning_strings: Vec<&'a str> = winners.iter().map(|h| h.input_string).collect();
    return Some(winning_strings);
}

fn parse_hand<'a>(hand_str: &'a str) -> Hand {
    // parse cards
    let mut cards: Vec<_> = hand_str
        .clone()
        .split_whitespace()
        .map(|card| parse_card(card))
        .collect();

    let profile: HandProfile = profile_hand(&mut cards);

    let (tie_breaker, hand_type) = determine_hand_type(profile, cards);

    return Hand {
        tie_breaker: tie_breaker,
        input_string: hand_str,
        hand_type: hand_type,
    };
}

fn determine_hand_type(profile: HandProfile, cards: Vec<Card>) -> (Vec<u8>, HandType) {
    let (suits, of_a_kinds, is_straight) = profile;
    let max_of_a_kind_count = of_a_kinds.values().max().unwrap_or(&0);

    let mut tie_breaker = Vec::new();
    let card_values: Vec<u8> = cards.iter().rev().map(|c| c.value).collect();
    let hand_type: HandType;

    if is_straight && suits.len() == 1 {
        // straight flush
        hand_type = HandType::StraightFlush;
        straight_tie_breaker(&cards, &mut tie_breaker);
    } else if *max_of_a_kind_count == 4 {
        // 4 of a kind
        hand_type = HandType::FourOfAKind;
        of_a_kind_tie_breaker(&of_a_kinds, &mut tie_breaker, &card_values);
    } else if *max_of_a_kind_count == 3 && of_a_kinds.len() == 2 {
        // full house
        hand_type = HandType::FullHouse;
        full_house_tie_breaker(&of_a_kinds, &mut tie_breaker);
    } else if suits.len() == 1 {
        // flush
        hand_type = HandType::Flush;
        tie_breaker.extend(card_values.iter());
    } else if is_straight {
        // straight
        hand_type = HandType::Straight;
        straight_tie_breaker(&cards, &mut tie_breaker);
    } else if *max_of_a_kind_count == 3 {
        // three of a kind
        hand_type = HandType::ThreeOfAKind;
        of_a_kind_tie_breaker(&of_a_kinds, &mut tie_breaker, &card_values);
    } else if of_a_kinds.len() == 2 {
        // two pair
        hand_type = HandType::TwoPair;
        two_pair_tie_breaker(&of_a_kinds, &mut tie_breaker, &card_values);
    } else if of_a_kinds.len() == 1 {
        // one pair
        hand_type = HandType::OnePair;
        of_a_kind_tie_breaker(&of_a_kinds, &mut tie_breaker, &card_values);
    } else {
        // high card
        hand_type = HandType::HighCard;
        tie_breaker.extend(card_values.iter());
    }
    (tie_breaker, hand_type)
}

fn two_pair_tie_breaker(
    of_a_kinds: &HashMap<u8, u8>,
    tie_breaker: &mut Vec<u8>,
    card_values: &Vec<u8>,
) {
    let mut keys: Vec<u8> = of_a_kinds.keys().cloned().collect();
    keys.sort();
    keys.reverse();

    for pair in keys {
        tie_breaker.push(pair);
    }

    tie_breaker.extend(card_values.iter());
}

fn full_house_tie_breaker(of_a_kinds: &HashMap<u8, u8>, tie_breaker: &mut Vec<u8>) {
    let mut pair_value = 0;
    let mut triple_value = 0;

    for (value, count) in of_a_kinds {
        if *count == 3 {
            triple_value = *value;
        }
        if *count == 2 {
            pair_value = *value;
        }
    }
    tie_breaker.push(triple_value);
    tie_breaker.push(pair_value);
}

fn of_a_kind_tie_breaker(
    of_a_kinds: &HashMap<u8, u8>,
    tie_breaker: &mut Vec<u8>,
    card_values: &Vec<u8>,
) {
    let value = of_a_kinds.keys().next().unwrap();
    tie_breaker.push(*value);
    tie_breaker.extend(card_values.iter());
}

fn straight_tie_breaker(cards: &Vec<Card>, tie_breaker: &mut Vec<u8>) {
    let mut highest_card = &cards.last().unwrap().value;

    // handle low ace
    if *highest_card == ACE_VALUE {
        // FIX ME assumes hand has 5 cards
        let second_card = &cards[3].value;
        if *second_card == 5 {
            highest_card = second_card;
        }
    }

    tie_breaker.push(*highest_card);
}

// characterise hand
// - count cards of a kind
// - identify straights and flushes
fn profile_hand(cards: &mut Vec<Card>) -> HandProfile {
    cards.sort_by_key(|c| c.value);
    let mut prev_value: u8 = 0;

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
        if prev_value + 1 != card.value && prev_value != 0 {
            // - check for low ace
            if prev_value != 5 && card.value != ACE_VALUE {
                is_straight = false;
            }
        }

        prev_value = card.value;
    }
    (suits, of_a_kinds, is_straight)
}

fn parse_card(card_str: &str) -> Card {
    let mut card_chars = card_str.chars();
    let suit_char = card_chars.next_back();

    let suit = match suit_char {
        Some('C') => Suit::Club,
        Some('S') => Suit::Spade,
        Some('H') => Suit::Heart,
        Some('D') => Suit::Diamond,
        None => panic!("Malformed card string: {:?}", card_str),
        Some(_) => panic!("Malformed card string: {:?}", card_str),
    };

    let value = match card_chars.as_str().parse::<u8>() {
        Ok(v) => v,
        Err(_) => match card_chars.next() {
            Some('A') => ACE_VALUE,
            Some('K') => KING_VALUE,
            Some('Q') => QUEEN_VALUE,
            Some('J') => JACK_VALUE,
            _ => panic!("Malformed card string: {:?}", card_str),
        },
    };

    return Card {
        suit: suit,
        value: value,
    };
}
