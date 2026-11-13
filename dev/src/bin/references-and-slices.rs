fn first_word(text: &str) -> &str {
    for (i, c) in text.chars().enumerate() {
        if " ".contains(c) {
            return &text[..i]; // ptr + len; string literals = immutable string references = string slices
            // &String ptr -> (String) ptr + len + capacity
            // String ptr + len + capacity
        }
    }
    &text[..]
}

fn first_element(arr: &[i32]) -> &[i32] {
    &arr[..1]
}

fn main() {
    let text = "Hello, world".to_owned();
    println!("{}", first_word(&text));

    let arr = [1, 2, 3, 4, 5];
    println!("{:?}", first_element(&arr));
}
