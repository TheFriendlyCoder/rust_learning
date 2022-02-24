fn main() {
    //unknown_problem_scoping();
    sample();

    string_literal_example();
    string_slice_example();
}

fn unknown_problem_scoping() {
    let text = String::from("Hello, world!");
    {
        let fw = first_word(&text);
        println!("First word is '{}'", fw);

        // This next line doesn't work because 'fw' is an immutable reference
        // and apparently the clear() method needs to take a mutable reference
        // to the orginal 'text' to empty it, and Rust prevents you from having
        // references to both mutable and immutable references in the same scope
        //text.clear();
    }

    // From what I understand, it should be perfectly fine to use a mutable
    // reference here because it is outside the scope of the previoius scope
    // block, but it is still not valid for some reason. TBD

    //text.clear();
}
fn string_slice_example() {
    let s = "How much wood can a woodchuck chuck";
    // The slice "s[9..]" will return "wood can a woodchuck chuck", so
    // the first_word method should return "wood" here instead of "How"
    // Confirms that our first_word method still works even on slice types
    println!("First word is sub-slice is {}", first_word(&s[9..]));
}
fn string_literal_example() {
    // NOTE: the data type of 's' below is &str, an immutable
    // reference to a slice
    let s = "Hello World";
    println!(
        "Value of first word in string literal is {}",
        first_word(&s)
    );
}

fn sample() {
    let text = String::from("Hello, world!");
    println!("First word is {}", first_word(&text));
}
// The following 2 function signatures are similar, but the second one
// allows our helper method to work with not only String objects, but
// also string literals AND string slices as well. The last two are
// obvious because they are the same type, but the way String objects
// are handled has to do with "string coercion" mechanics which hasn't
// been taught yet
//fn first_word(s: &String) -> &str {
fn first_word(s: &str) -> &str {
    // Assuming our string is ASCII encoded, we can use
    // single-byte characters to locate the space character
    const SPACE: u8 = b' ';
    let bytes = s.as_bytes();

    // enumerate returns a tuple, with the first element being
    // the index of an item in a list, and the second a reference
    // to the item itself
    for (i, &item) in bytes.iter().enumerate() {
        if item == SPACE {
            // return a slice which contains characters starting with
            // the first and ending at offset i-1 (range indices are exclusive
            // on the upper bound)
            return &s[..i];

            // ranges without a starting index assume 0, so the previous line
            // would be equivalent to:
            // return &s[0..i];
        }
    }

    // If we get here we know there are no spaces in the string so return
    // the entire word. A range with no starting value assumes index 0, and
    // a range with no ending value assumes string.len() offset, so the range
    // [..] will return a slice that includes a reference to the entire string
    &s[..]
}
