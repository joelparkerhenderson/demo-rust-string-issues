//! Demonstration of Rust string issues,
//! such as UTF-8 encoding, accents, etc.
//!
//! Feedback welcome. 
//!
//!
//! ## References
//!
//! For full instructions see the source code repository:
//! https://github.com/joelparkerhenderson/demo-rust-string-issues
//!
//! Annotations below come mostly from the article by Mortoray:
//! https://mortoray.com/2013/11/27/the-string-type-is-broken/
//!
//!
//! ## Tracking
//!
//! * Package: demo-rust-string-issues
//! * Version: 1.1.0
//! * Updated: 2021-06-23T02:35:54Z
//! * License: BSD or GPL or MIT
//! * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>

use unicode_segmentation::UnicodeSegmentation;

/// Print various information about a given string.
///
/// The purpose is to show the difference between a string's bytes,
/// characters, graphemes, and related Unicode representation issues.
///
fn demo(message: &str, s: &str) {
    println!("\n{}\n", message);
    println!("String: {}", s);
    println!("String length: {}", s.len());
    println!("Characters debug: {:?}", s.chars());
    println!("Characters count: {}", s.chars().count());
    println!("Characters reverse: {}", s.chars().rev().collect::<String>());
    println!("Graphemes vector: {:?}", s.graphemes(true).collect::<Vec<&str>>());
    println!("Graphemes count: {}", s.graphemes(true).count());
    println!("Graphemes reverse: {}", s.graphemes(true).rev().collect::<String>());
    println!("Lowercase: {}", s.to_lowercase());
    println!("Uppercase: {}", s.to_uppercase());
}

/// Demo of umlaut with noël
///
/// Using the text “noël” with a decomposed Unicode string “noe\u0308l”, I
/// checked the following:
///
/// Does it print correctly? Yes, most languages are capable of doing this.
/// Though the ideone.com interface seems to break the output (so be careful
/// with testing).
///
/// What is the reverse? “lëon”, correct? Mostly this fails. The most common
/// result is “l̈eon” (the dieresis is on the ‘l’ instead of the ‘e’). This is
/// what happens without a string class, by just reversing an array of code
/// points.
///
/// What are the first three characters? Mostly the answer here is “noe”, as
/// opposed to the desired “noë”. This could easily lead into a big discussion
/// about what a character is, but I assume most people would not be happy with
/// the current result. This is again indicative of a string type which merely
/// treats the data as an array of code points.
///
/// What is the length? The common answer is 5. And yet again, this indicates
/// our string types are merely arrays of characters and not truly handling the
/// text.
///
/// For all of these questions, try to consider what should happen if you were
/// editing this text in your favourite word processor or text editor. I
/// generally expect that the ‘ë’ character is handled as a single entity. I
/// don’t expect backspace/delete to just remove part of the letter. I expect
/// copying the first three letters to include the accent.
///
/// A final check I did was to compare two logically equivalent strings with
/// different composition forms. Here “noël” is using the precomposed “ë”
/// character.
///
/// Is precomposed == decomposed? The answer is no in all tests. However,
/// several languages do offer Unicode normalization libraries. In those
/// languages the normal form of the strings does compare equal. JavaScript does
/// not have such a library, which is really tragic because it’s primarily a UI
/// language, exactly where’d you want proper unicode functionality.
///
/// It’s tempting to argue that normalization and lexical analysis is not part
/// of the basic string type. But these seem like fundamental operations one
/// would want to do with text. If they aren’t included, what exactly is the
/// purpose of the string type?
///
pub fn demo_of_umlaut() {
    demo("Demo of umlaut with noël with precomposed characters", "noël");
    demo("Demo of umlaut with noël with decomposed characters", "noe\u{0308}l");
}

/// Demo of virguilla with mañana
///
/// This function is much like the above `demo_of_umlaut`. The purpose is to 
/// show that the issues of precomposed forms vs. decomposed forms affect more
/// than just the umlaut.
///
pub fn demo_of_virguilla() {
   demo("Demo of virguilla with mañana with precomposed characters", "mañana");
    demo("Demo of virguilla with mañana with decomposed characters", "man\u{0303}ana");
}

/// Demo of emoticons with 😸😾
///
/// Unicode has cats in it. I hope you have a font which shows them — if not,
/// the title of this section is a happy cat and a sad cat, part of the Unicode
/// emoticon set). These characters were chosen since they are outside of the
/// BMP (basic multilingual plane). This spells trouble for languages using
/// UTF-16 encodings (Java, C#, JavaScript).
///
/// What is the length? Python unicode correctly reports 2. Those UTF-16
/// languages tend to report 4. The characters require surrogate pairs.
///
/// What is the substring after the first character? Python unicode correctly
/// reports the sad cat “😾”. The UTF-16 languages produce invalid strings with
/// a half-surrogate followed by the sad cat.
///
/// What is the reverse? Python unicode gets the correct reverse of “😾😸”. The
/// UTF-16 languages produce invalid strings. With C# I think I uncovered a
/// defect in ideone. It doesn’t even show the invalid string and instead shows
/// no output at all for the entire program! [ideone defect]
///
/// Languages using an encoding agnostic library, like C++, Perl, and normal
/// Python 2 strings, fail here as well. They ignore any encoding and assume the
/// string is an array of 1-byte code points. Python 3 adopted unicode as the
/// default string type, thus fixing some problems. It appears that Perl also
/// has a ‘utf8’ mode which fixes problems for these cats, but not for the“noël”
/// string.
///
pub fn demo_of_emoticons() {
    demo("Demo of emoticons with 😸😾", "😸😾");
}

/// Demo of ligature with "baﬄe"
//
/// This string contains a ligature character, the “ffl” part is a single
/// unicode code point. They exist mainly for compatibility, but they are a good
/// test for case conversion.
///
/// What is the uppercase? I did not find any language which doesn’t print
/// “BAﬄE”. Notice the ligature remains lowercase. The expected answer is of
/// course“BAFFLE”.
///
/// Unicode has a special class of case conversion: this single ligature code
/// point is actually converted to three code points. By not following these
/// additional rules, a language uppercase function produces an interesting
/// result: a string converted to uppercase still has lowercase characters in
/// it.
///
pub fn demo_of_ligature() {
    demo("Demo of ligature with baﬄe", "baﬄe");
}

fn main() {
    demo_of_umlaut();
    demo_of_virguilla();
    demo_of_emoticons();
    demo_of_ligature();
}
