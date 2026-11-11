fn main() {
    let mut names = Vec::new();
    let name = String::from("Ana");
    names.push(&name);
    println!("{name} joined the list");
    println!("{:?}", names);

    let slice = &name[1..2];
    println!("{}", slice);

    let mut str = String::from("hello");
    println!("{}", str.capacity()); // the total amount of bytes the string has received from the allocator
    str.push_str("Ana");
    println!("{}", str.capacity());

    let s1 = String::from("hello");
    let _s2 = s1; // shallow copy or move; s1 was dropped
    // println!("s1 = {s1}"); is not possible

    let original = String::from("draft");
    let _edited = original.clone(); // a deep copy; without clone(), the ptr, len and capacity will be moved from the original to the edited variable

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}"); // compiles successfully
}
