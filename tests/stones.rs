
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
#[should_panic]
fn token_is_color_false() {
    assert!(Token::Right.is_color());
}

#[test]
fn token_is_direction_true() {
    assert!(Token::Right.is_direction());
}

#[test]
#[should_panic]
fn token_is_direction_false() {
    assert!(Token::Red.is_direction());
}

#[test]
fn token_is_number_true() {
    assert!(Token::One.is_number());
}

#[test]
#[should_panic]
fn token_is_number_false() {
    assert!(Token::Red.is_number());
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

#[test]
fn val_same_type() {
    assert!(Value::same_type(&Value::Num(0), &Value::Num(3)));
}

#[test]
#[should_panic]
fn val_same_type_false() {
    assert!(Value::same_type(&Value::Num(0), &Value::Bool(true)));
}

#[test]
fn val_get_num() {
    assert!(Value::Num(3).get_num() == 3);
}

#[test]
#[should_panic]
fn val_get_not_num() {
    Value::Num(3).get_arr();
}

#[test]
fn val_eq() {
    assert!(Value::Num(3) == Value::Num(3));
}

#[test]
#[should_panic]
fn val_not_eq() {
    assert!(Value::Num(3) == Value::Num(2));
}

#[test]
#[should_panic]
fn val_types_not_eq() {
    assert!(Value::Num(3) == Value::Bool(false));
}

#[test]
fn val_num_gt() {
    assert!(Value::Num(3) > Value::Num(2));
}

#[test]
fn val_num_le() {
    assert!(Value::Num(2) <= Value::Num(3));
}

#[test]
fn val_arr_lt() {
    assert!(Value::Arr(vec![]) < Value::Arr(vec![Value::Num(1)]));
}

