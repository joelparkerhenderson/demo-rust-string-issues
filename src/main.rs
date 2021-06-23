use unicode_segmentation::UnicodeSegmentation;

/// Demonstration of Rust string issues,
/// such as UTF-8 encoding, accents, etc.
///
/// Feedback welcome. For full instructions see link below.
///
/// https://github.com/joelparkerhenderson/demo-rust-string-issues
///
///
/// Tracking:
///
/// * Package: demo-rust-string-issues
/// * Version: 1.1.0
/// * Updated: 2021-06-23T02:35:54Z
/// * License: BSD or GPL or MIT
/// * Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>

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

}

fn main() {
    demo("Demo of noël with precomposed characters", "noël");
    demo("Demo of noël with decomposed characters", "noe\u{0308}l");
    demo("Demo of mañana with precomposed characters", "mañana");
    demo("Demo of mañana with decomposed characters", "man\u{0303}ana");
    demo("Demo of cats with precomposed characters", "😸😾");
}
