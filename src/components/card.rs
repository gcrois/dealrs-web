use dealrs::deck::{Card, Rank, Suit};
use dioxus::prelude::*;

// Base spritesheet tile size (source pixels)
const TILE_W: i32 = 56;
const TILE_H: i32 = 80;

// Full spritesheet image
const CARD_SPRITE: Asset = asset!("/assets/cards.png");

/// Which card to render.
#[derive(Clone, PartialEq)]
pub enum CardKind {
    Normal(Card),
    /// Specials live in column 0 (different rows).
    Special(Special),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Special {
    EmptyCard,   // row 0
    EmptySpace,  // row 1
    RedBack,     // row 2
    BlueBack,    // row 3
    Joker,       // row 4
    JokerAlt,    // row 5
}

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    pub kind: CardKind,
    #[props(default)]
    pub class: Option<String>,
}

#[component]
pub fn CardComponent(props: CardProps) -> Element {
    let CardProps { kind, class } = props;

    let (row, col): (i32, i32) = match kind {
        CardKind::Normal(c) => (suit_row(c.suit()), rank_col(c.rank())),
        CardKind::Special(s) => (special_row(s), 0),
    };

    // Constants for grid
    let cols = 14; // 0..13
    let rows = 6;  // specials

    // Inline CSS
    // Inline CSS for front and back faces
    let front_style = format!(
        "
        background-image: url('{}'); \
        background-repeat: no-repeat; \
        image-rendering: pixelated; \
        background-size: {}% {}%; \
        background-position: {}% {}%;
        ",
        CARD_SPRITE,
        cols * 100,
        rows * 100,
        (col as f32 / (cols - 1) as f32) * 100.0,
        (row as f32 / (rows - 1) as f32) * 100.0,
    );

    // Default back uses the red back tile (column 0)
    let back_row = special_row(Special::RedBack);
    let back_col = 0;
    let back_style = format!(
        "
        background-image: url('{}'); \
        background-repeat: no-repeat; \
        image-rendering: pixelated; \
        background-size: {}% {}%; \
        background-position: {}% {}%;
        ",
        CARD_SPRITE,
        cols * 100,
        rows * 100,
        (back_col as f32 / (cols - 1) as f32) * 100.0,
        (back_row as f32 / (rows - 1) as f32) * 100.0,
    );

    let wrapper_style = format!("aspect-ratio: {}/{}; display: inline-block;", TILE_W, TILE_H);
    let wrapper_class = format!("card-wrapper {}", class.as_deref().unwrap_or_default());

    rsx! {
        div {
            class: "{wrapper_class}",
            style: "{wrapper_style}",
            div {
                class: "card-inner",
                div { class: "card-face card-front", style: "{front_style}" },
                div { class: "card-face card-back", style: "{back_style}" },
            }
        }
    }
}

// ---- mapping helpers ----

#[inline]
fn suit_row(s: Suit) -> i32 {
    // row 0: Hearts, 1: Spades, 2: Diamonds, 3: Clubs
    match s {
        Suit::Hearts => 0,
        Suit::Spades => 1,
        Suit::Diamonds => 2,
        Suit::Clubs => 3,
    }
}

#[inline]
fn rank_col(r: Rank) -> i32 {
    // col 0 reserved for specials
    // col 1: Ace
    // col 2..13: Two..King
    match r {
        Rank::Ace   => 1,
        Rank::Two   => 2,
        Rank::Three => 3,
        Rank::Four  => 4,
        Rank::Five  => 5,
        Rank::Six   => 6,
        Rank::Seven => 7,
        Rank::Eight => 8,
        Rank::Nine  => 9,
        Rank::Ten   => 10,
        Rank::Jack  => 11,
        Rank::Queen => 12,
        Rank::King  => 13,
    }
}

#[inline]
fn special_row(s: Special) -> i32 {
    match s {
        Special::EmptyCard  => 0,
        Special::EmptySpace => 1,
        Special::RedBack    => 2,
        Special::BlueBack   => 3,
        Special::Joker      => 4,
        Special::JokerAlt   => 5,
    }
}
