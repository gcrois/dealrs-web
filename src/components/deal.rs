use dioxus::prelude::*;
use dealrs::{
    hand::Hand,
    deck::{sample_cards_ordered, Card, CardMask}, hand::Best5, rng_from_seed
};

use crate::components::{CardComponent, CardKind};

#[derive(Clone, Debug)]
struct HandResult {
    cards: Vec<Card>,
    best_mask: CardMask,
    hand_type: Hand,
}

impl HandResult {
    fn new(cards: Vec<Card>) -> Option<Self> {
        let (best_mask, hand_type) = dealrs::hand::refbest5::RefBest5::new()
            .best5(CardMask::from_multi(cards.as_slice()));
        
        Some(HandResult {
            cards,
            best_mask,
            hand_type,
        })
    }
    
    fn hand_color_class(&self) -> &'static str {
        match self.hand_type {
            Hand::StraightFlush { .. } => "bg-purple-100 border-purple-400",
            Hand::FourOfAKind { .. } => "bg-red-100 border-red-400",
            Hand::FullHouse { .. } => "bg-orange-100 border-orange-400", 
            Hand::Flush { .. } => "bg-blue-100 border-blue-400",
            Hand::Straight { .. } => "bg-green-100 border-green-400",
            Hand::ThreeOfAKind { .. } => "bg-yellow-100 border-yellow-400",
            Hand::TwoPair { .. } => "bg-indigo-100 border-indigo-400",
            Hand::OnePair { .. } => "bg-gray-100 border-gray-400",
            Hand::HighCard { .. } => "bg-slate-50 border-slate-300",
        }
    }
    
    fn rank_badge_class(&self) -> &'static str {
        match self.hand_type {
            Hand::StraightFlush { .. } => "bg-purple-500 text-white",
            Hand::FourOfAKind { .. } => "bg-red-500 text-white",
            Hand::FullHouse { .. } => "bg-orange-500 text-white", 
            Hand::Flush { .. } => "bg-blue-500 text-white",
            Hand::Straight { .. } => "bg-green-500 text-white",
            Hand::ThreeOfAKind { .. } => "bg-yellow-500 text-black",
            Hand::TwoPair { .. } => "bg-indigo-500 text-white",
            Hand::OnePair { .. } => "bg-gray-500 text-white",
            Hand::HighCard { .. } => "bg-slate-400 text-white",
        }
    }

    fn hand_display_name(&self) -> String {
        match &self.hand_type {
            Hand::StraightFlush { top } => format!("Straight Flush ({})", top),
            Hand::FourOfAKind { quad, .. } => format!("Four of a Kind ({}s)", quad),
            Hand::FullHouse { trip, pair } => format!("Full House ({}s over {}s)", trip, pair),
            Hand::Flush { .. } => "Flush".to_string(),
            Hand::Straight { top } => format!("Straight ({})", top),
            Hand::ThreeOfAKind { trip, .. } => format!("Three of a Kind ({}s)", trip),
            Hand::TwoPair { .. } => "Two Pair".to_string(),
            Hand::OnePair { pair, .. } => format!("Pair of {}s", pair),
            Hand::HighCard { .. } => "High Card".to_string(),
        }
    }
}

#[component]
pub fn Deal() -> Element {
    // controls
    let mut num_hands = use_signal(|| 3usize);
    let mut cards_per_hand = use_signal(|| 7usize);
    let mut seed = use_signal(String::new);

    // results
    let mut hand_results = use_signal::<Vec<HandResult>>(Vec::new);

    let deal_now = move |_| {
        let hands = *num_hands.read();
        let cards_each = *cards_per_hand.read();
        
        // rng_from_seed takes Option<String>; empty -> None for nondeterministic RNG
        let seed_opt = {
            let s = seed.read().clone();
            if s.is_empty() { None } else { Some(s) }
        };
        let mut rng = rng_from_seed(seed_opt);

        
        // Split into hands and evaluate each
        let mut results = Vec::new();
        for _ in 0..hands {
            let hand_cards = sample_cards_ordered(CardMask::full(), cards_each, &mut rng);

            if let Some(result) = HandResult::new(hand_cards) {
                results.push(result);
            }
        }
        
        // Sort by hand strength using built-in PartialOrd (best first)
        results.sort_by(|a, b| b.hand_type.partial_cmp(&a.hand_type).unwrap_or(std::cmp::Ordering::Equal));
        hand_results.set(results);
    };

    // Helper function to check if a card is in the best 5 for this hand
    let is_best_card = |card: &Card, hand_result: &HandResult| -> bool {
        hand_result.best_mask.iter().any(|c| c == *card)
    };

    rsx! {
        div { class: "p-4 space-y-4",
            h2 { class: "text-xl font-semibold", "Deal and rank hands" }

            div { class: "flex gap-3 items-end flex-wrap",
                div { class: "flex flex-col",
                    label { r#for: "num_hands", class: "text-sm", "Number of hands" }
                    input {
                        id: "num_hands",
                        r#type: "number",
                        min: "1",
                        value: num_hands.read().to_string(),
                        oninput: move |ev| {
                            if let Ok(v) = ev.value().parse::<usize>() {
                                num_hands.set(v);
                            }
                        }
                    }
                }

                div { class: "flex flex-col",
                    label { r#for: "cards_per_hand", class: "text-sm", "Cards per hand" }
                    input {
                        id: "cards_per_hand",
                        r#type: "number",
                        min: "1",
                        value: cards_per_hand.read().to_string(),
                        oninput: move |ev| {
                            if let Ok(v) = ev.value().parse::<usize>() {
                                cards_per_hand.set(v);
                            }
                        }
                    }
                }

                div { class: "flex flex-col grow",
                    label { r#for: "seed", class: "text-sm", "Seed (optional, deterministic)" }
                    input {
                        id: "seed",
                        r#type: "text",
                        value: seed.read().as_str(),
                        oninput: move |ev| seed.set(ev.value()),
                        placeholder: "leave blank for random"
                    }
                }

                button {
                    class: "px-4 py-2 rounded-md border bg-blue-500 text-white hover:bg-blue-600 font-medium",
                    onclick: deal_now,
                    "Deal & Rank"
                }
            }

            if hand_results.read().is_empty() {
                p { class: "text-sm text-gray-500", "No hands dealt yet." }
            } else {
                div { class: "space-y-3",
                    h3 { class: "text-lg font-semibold", "Hands ranked by strength (best first):" }
                    
                    {hand_results.read().iter().enumerate().map(|(rank_idx, hand_result)| {
                        let border_class = hand_result.hand_color_class();
                        let badge_class = hand_result.rank_badge_class();
                        let display_name = hand_result.hand_display_name();
                        
                        rsx! {
                            div { 
                                key: format!("hand-{rank_idx}"),
                                class: "border-2 rounded-lg p-4 {border_class}",
                                
                                div { class: "flex items-center justify-between mb-3",
                                    div { class: "flex items-center gap-3",
                                        span { class: "text-2xl font-bold text-gray-600", "#{rank_idx + 1}" }
                                        span { 
                                            class: "px-3 py-1 rounded-full text-sm font-semibold {badge_class}",
                                            "{display_name}"
                                        }
                                    }
                                }
                                
                                div { class: "flex flex-wrap gap-2 justify-center",
                                    {hand_result.cards.iter().enumerate().map(|(card_idx, card)| {
                                        let is_highlighted = is_best_card(card, hand_result);
                                        let card_class = if is_highlighted {
                                            "inline-block w-16 align-middle ring-2 ring-yellow-400 shadow-md transform scale-105"
                                        } else {
                                            "inline-block w-16 align-middle opacity-60"
                                        };
                                        
                                        rsx! {
                                            div {
                                                key: format!("{rank_idx}-{card_idx}-{:?}", card),
                                                CardComponent {
                                                    kind: CardKind::Normal(card.clone()),
                                                    class: Some(card_class.into()),
                                                }
                                            }
                                        }
                                    })}
                                }
                                
                                p { class: "text-center text-sm text-gray-600 mt-2",
                                    "Best 5 cards highlighted"
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}