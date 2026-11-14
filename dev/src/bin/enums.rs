#![allow(unused)]

use crate::Coin::Quarter;

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

impl Message {
    fn call(&self) {}
}

fn route(ip_kind: &IpAddrKind) {}

#[derive(Debug)]
enum UsState {
    Alaska,
    Alabama,
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1819,
            UsState::Alabama => year >= 1959,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quater from {state:?}");
                25
            }
        }
    }

    fn describe_state_quarter(&self) -> Option<String> {
        let Coin::Quarter(state) = self else {
            return None;
        };

        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("::1"));

    route(&four);
    dbg!(&four);

    let m = Message::Write(String::from("hello"));
    m.call();
    dbg!(&m);

    let coin = Coin::Penny;
    let cents = Coin::value_in_cents(&coin);
    // let cents = coin.value_in_cents(); // automatic referencing behaviour
    assert_eq!(1, cents);
    let coin = Coin::Quarter(UsState::Alabama);
    let cents = Coin::value_in_cents(&coin);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = rand::random_range(0..10);
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // catch-all pattern
        _ => (),
    }

    let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    let mut count = 0u32;
    if let Coin::Quarter(state) = &coin {
        println!("State quater from {state:?}")
    } else {
        count += 1;
    }

    println!("{:?}", coin.describe_state_quarter());
}
