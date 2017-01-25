
extern crate stones;
use stones::*;

#[test]
fn token_from_stone() {
    assert_eq!(Token::from_stone(Color::Red), Token::Red);
}

#[test]
fn token_from_direction() {
    assert_eq!(Token::from_direction(Direction::Up), Token::Up);
}

#[test]
fn token_is_color_true() {
    assert!(Token::Red.is_color());
}

#[test]
fn token_is_color_false() {
    assert!(!Token::Right.is_color());
}

#[test]
fn token_is_direction_true() {
    assert!(Token::Right.is_direction());
}

#[test]
fn token_is_direction_false() {
    assert!(!Token::Red.is_direction());
}

#[test]
fn token_is_number_true() {
    assert!(Token::One.is_number());
}

#[test]
fn token_is_number_false() {
    assert!(!Token::Red.is_number());
}

#[test]
fn stone_weight_gt() {
    assert!(Color::Blue > Color::Red);
}

#[test]
fn stone_weight_lt() {
    assert!(Color::Red < Color::Blue);
}

#[test]
fn stone_weight_eq() {
    assert!(Color::Red == Color::Red);
}

#[test]
fn stone_invis_lightest() {
    assert!(Color::Invis < Color::Red);
}

