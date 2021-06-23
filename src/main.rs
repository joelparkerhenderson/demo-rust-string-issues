/// Demonstration of Rust string issues,
/// such as UTF-8 encoding, accents, etc.
///
/// Feedback welcome.
///
/// This repo is based on the artice about string handling here:
/// See https://mortoray.com/2013/11/27/the-string-type-is-broken/
///
///
/// This code is tested on Rust 1.5.3:
///
/// ```sh
/// $ rustc --version                                                                     
/// rustc 1.53.0
/// ```
///
/// Tracking:
///
/// * Package: demo-rust-string-issues
/// * Version: 1.1.0
/// * Updated: 2021-06-23T02:35:54Z
/// * License: BSD or GPL or MIT
/// * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>

fn noel() {
    let s = String::from("noël");

    // Does it print correctly? Yes, most languages are capable of doing this.
    println!("Does it print correctly? {}", s);

    // What is the length? The common answer is 5. And yet again, this indicates
    // our string types are merely arrays of characters and not truly handling
    // the text.
    println!("What is the character length? {}", s.chars().count());

    // What is the reverse? “lëon”, correct? Mostly this fails. The most common
    // result is “l̈eon” (the dieresis is on the ‘l’ instead of the ‘e’). This
    // is what happens without a string class, by just reversing an array of
    // code points.
    println!("What is the reverse? {}", s.chars().rev().collect::<String>());

    // What are the first three characters? Mostly the answer here is “noe”, as
    // opposed to the desired “noë”. This could easily lead into a big
    // discussion about what a character is, but I assume most people would not
    // be happy with the current result. This is again indicative of a string
    // type which merely treats the data as an array of code points.
    println!("What are the first three characters? {}", s.chars().take(3).collect::<String>());
}

fn cats() {
    let s = String::from("😸😾");

    // Does it print correctly? Yes, most languages are capable of doing this.
    println!("Does it print correctly? {}", s);

    // What is the reverse? Python unicode gets the correct reverse of “😾😸”.
    // The UTF-16 languages produce invalid strings. With C# I think I uncovered
    // a defect in ideone. It doesn’t even show the invalid string and instead
    // shows no output at all for the entire program! [ideone defect]
    println!("What is the reverse? {}", s.chars().rev().collect::<String>());

    // What is the length? Python unicode correctly reports 2. Those UTF-16
    // languages tend to report 4: the characters require surrogate pairs.
    println!("What is the character length? {}", s.chars().count());

    // What is the substring after the first character? Python unicode correctly
    // reports the sad cat “😾”. The UTF-16 languages produce invalid strings
    // with a half-surrogate followed by the sad cat.
    println!("What is the substring after the first character? {}", s.chars().skip(1).collect::<String>());

}
fn main() {
    noel();
    cats();
}
