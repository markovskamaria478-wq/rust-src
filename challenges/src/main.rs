pub const MAX: u32 = u32::MAX;
pub const MIN: i32 = i32::MIN;


fn main() {
    // task_01
    let score: u8 = 100;
    let score= score as char;
    let score: String = score.to_string();
    let score: &str = score.as_str();
    println!("{}", score);

    let mut points = 10;
    points += 5;
    assert_eq!(points, 15);

    // task_02
    let data = (10, 20, 30, 40);
    let (first_el, .., last_el) = data;
    assert_eq!((first_el, last_el), (10, 40));

    // task_03
    let size_of_u64 = std::mem::size_of::<u64>();
    let size_of_i128 = std::mem::size_of::<i128>();
    println!("size of u64 - {} bits", size_of_u64 * 8);
    println!("size of i128 - {} bits", size_of_i128 * 8);

    assert_eq!(MAX, 4294967295);
    assert_eq!(MIN, -2147483648);

    // task_04
    let val: u8 = 200;
    let big_val = val as u32;
    let res = u32::checked_add(big_val, 100).unwrap();
    assert_eq!(res, 300);
    
    let res_overflow = u8::checked_add(u8::MAX, 1).unwrap_or(0);
    assert_eq!(res_overflow, 0);

    // task_05
    let mut factorial: i32 = 1;
    for x in 1..5 {
        factorial *= x;
    }
    assert_eq!(factorial, 24);

    for c in 'A'..='E' {
        println!("{} -> {}", c, c as u8);
    }
}