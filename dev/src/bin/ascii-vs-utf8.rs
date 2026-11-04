fn additional_info(string: &str) {
    println!("string: {}", string);
    println!("indexes, byte_indexes, chars, bytes, bytes");
    for (index, (byte_index, c)) in string.char_indices().enumerate() {
        println!(
            "{:02} {:02} {:?} {} {:?}",
            index,
            byte_index,
            c,
            c as u32,
            c.to_string().as_bytes()
        );
    }
}

fn find_vowels_as_chars() {
    let now = std::time::Instant::now();
    let mut vowels = 0;
    let text = std::fs::read_to_string("/home/maria/Documents/english/conspectus.txt").unwrap();
    for c in text.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            vowels += 1;
        }
    }
    println!("{} vowels were found in {:?}", vowels, now.elapsed());
}

fn find_vowels_as_bytes() {
    let now = std::time::Instant::now();
    let mut vowels = 0;
    let text = std::fs::read_to_string("/home/maria/Documents/english/conspectus.txt").unwrap();
    for c in text.bytes() {
        if b"aeiou".contains(&c) {
            vowels += 1;
        }
    }
    println!("{} vowels were found in {:?}", vowels, now.elapsed());
}

fn main() {
    let text_ascii = "Hello, world";
    additional_info(text_ascii);

    let text_utf8 = "世界! 🌍";
    additional_info(text_utf8);

    find_vowels_as_chars();
    find_vowels_as_bytes();
}
