/*
checked_, saturating_, wrapped_, overflowing_, T::try_from,
std::num::{Wrapping, Saturating}
*/

use std::num::{Saturating, Wrapping};

fn withdraw(balance: u64, amount: u64) -> Option<u64> {
    u64::checked_sub(balance, amount)
}

#[allow(unused_variables)]
fn main() {
    let balance: u64 = 200;

    for amount in [30, 250] {
        match withdraw(balance, amount) {
            Some(left) => println!("balance is now {left}"),
            None => println!("can't withdraw {amount}: not enough money"),
        }
    }

    let mut ticket: u8 = 254;
    for _ in 0..4 {
        println!("your ticket is {}", ticket);
        ticket = ticket.wrapping_add(1);
    }

    let (value, overflowed) = u8::overflowing_add(250, 10);
    println!("value: {value}, overflowed: {overflowed}");
    let (value, overflowed) = 200u8.overflowing_sub(50);
    println!("value: {value}, overflowed: {overflowed}");

    let x: u8 = u8::wrapping_add(240, 10);

    let x = 300;
    match u8::try_from(x) {
        Ok(small) => println!("fits: {small}"),
        Err(e) => println!("300 doesn't fit in a u8: {e}"),
    }

    let mut a = Wrapping(255u8);
    a += 1;
    println!("a is equal to {a}");
    let mut b = Saturating(100u8);
    b += 200;
    println!("b is equal to {b}");
}
