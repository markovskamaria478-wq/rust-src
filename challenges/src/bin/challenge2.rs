#[allow(unused)]
use std::{
    any::{self, type_name_of_val},
    char::from_u32,
    fmt::Error,
    io::{self, read_to_string, stdin},
    path::absolute,
};

fn how_many_bytes<T>(_val: T) -> usize {
    let size_of = std::mem::size_of::<T>();
    size_of
}

#[allow(dead_code)]
fn add_points(score: u8, bonus: u8) -> u8 {
    score + bonus
}

fn main() {
    // task_01
    let x = 5;
    let x = x * 2;
    {
        let x = x + 1;
        assert_eq!(x, 11);
    }
    assert_eq!(12, x + 2);

    // task_02
    let x = 1;
    let y = 2;
    let (x, y) = (y, x);
    assert_eq!((x, y), (2, 1));

    // task_03
    let arr = [0, 20, 30, 40, 50];
    let [first_el, .., last_el] = arr;
    let tup = (first_el, last_el);
    assert_eq!(tup, (0, 50));

    // task_04
    let x = 1;
    println!("{}", how_many_bytes(x) * 8);

    // task_05
    let prediction: [&str; 4] = ["i32", "f64", "&str", "bool"];
    let values = (1, 1.0, "a", true);
    let type_name_of_values: [&str; 4] = [
        type_name_of_val(&values.0),
        type_name_of_val(&values.1),
        type_name_of_val(&values.2),
        type_name_of_val(&values.3),
    ];
    assert_eq!(type_name_of_values, prediction);

    // task_06
    let distinct_values =
        i32::checked_add(i16::MAX as i32, i32::checked_abs(i16::MIN as i32).unwrap()).unwrap();
    println!("The answer is {}", distinct_values + 1);

    // task_07
    /*
    let x: u8 = 250 + 10; when the compiler can see the whole calculation, it refuses to build it
    let x = add_points(250, 10); won't work without the --release option; in debug builds, it panicks
    */
    let x = 300;
    let x = x as u8;
    assert_eq!(x, 44); // silently truncates and wraps to a smaller value 300 mod 256
    // let x = u8::try_from(300_i32);
    // assert!(x.is_err());
    // println!("{}", 300_.0_f64 as u8);
    // println!("{}", 300.99_f64.round() as i32);
    // println!("{}", 65_u8 as char);
    // let c = char::from_u32(65);
    // println!("{}", c.unwrap());
    // let text = "42";
    // let text: i32 = text.parse().expect("Not a valid number");
    // let val = 42.0_f64;
    // let val = val.to_string() + "!";
    // println!("{}", type_name_of_val(&val));

    // let (score_1, score_2, score_3) = (7, 8, 10);
    // let sum = score_1 + score_2 + score_3;
    // let avg = sum as f64 / 3.0;
    // println!("{:.2}", avg);

    // let price = "4.50";
    // let quantity = 3_i32;
    // let price: f64 = price.parse().expect("Not a valid price");
    // println!("total: {}", price * quantity as f64);
    let x = 256;
    let x = x as u8;
    assert_eq!(x, 0); // 256 mod 256
    let y = -1;
    let y = y as u8;
    assert_eq!(y, 255); // -1 mod 256
    
    // Rust Integer Overflow
    let score: u8 = 250;
    println!("{}", score.wrapping_add(10)); // 
}
