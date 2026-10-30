use std::any::type_name_of_val;
#[allow(unused_variables)]

fn main() {
    let x = 1;

    let (mut x, y) = (2, 3);
    x += 1;
    assert_eq!(x, 3);
    
    let (x, y);
    (x,..) = (1, 2);
    [..,y] = [3, 4];
    assert_eq!([x, y], [1, 4]);

    let size_of_us = std::mem::size_of::<usize>();
    let size_of_is = std::mem::size_of::<isize>();
    println!("the size of usize in bits is {}", size_of_us*8);
    println!("the size of isize in bits is {}", size_of_is*8);

    let x: u16 = 32_u8 as u16;
    assert_eq!("u16", type_name_of_val(&x));

    assert_eq!(u8::MAX, 255);
    assert_eq!(i8::MAX, 127);

    let x: u16 = 255_u16 + 4_u16;
    let x = u16::checked_add(255, 4).unwrap();

    assert!(0.1_f32 + 0.2_f32 == 0.3_f32);

    let mut sum: i32 = 0;
    for i in -3..2 { // the end point of the range is excluded 
        sum += i
    }
    assert!(sum==-5);
    for c in 'a'..='z' {
        print!("{} ", c as u8);
    }
    println!("")
}