fn main() {
    let mut s = String::from("hello world");

    // string slices
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s); // world will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now invalid.
}

fn first_word(s: &string) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
