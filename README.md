# Demonstration of Rust string issues

Demonstration of Rust string issues,
such as UTF-8 encoding, accents, etc.

Feedback welcome.


Articles:

* [The string type is broken - by Edaqa Mortoray](https://mortoray.com/2013/11/27/the-string-type-is-broken/)

* [JavaScript has a Unicode problem - by Mathais Bynens](https://mathiasbynens.be/notes/javascript-unicode)

Crates that can help:

* [unicode_segmentation](https://crates.io/crates/unicode_segmentation)

* [unicode_reverse](https://crates.io/crates/unicode_reverse)

Thanks:

* [Hacker News](https://news.ycombinator.com/item?id=27599243) and user [ridiculous_fish](https://news.ycombinator.com/user?id=ridiculous_fish).


## Issues


### Decomposed characters

Rust can represent some characters as either "precomposed form" or "decomposed form".

* Precomposed form is one code point. For example "ë" may be represented as one code point U+00EB.

* Decomposed form is multiple code points that combine. For example "ë" may be represented as code points U+0065 U+0308, aka the letter "e" followed by the combining umlaut accent. Decomposed form is aka a "combining sequence" because the diaeresis is combining with the base character.

Many computers render identically for the precomposed form and decomposed form.

* This means a user can't visually see the difference.

* You more or less need a hex editor to figure out which form you've got.

One could reasonably conclude that precomposed forms are just better and easier.

* But precomposed forms are considered legacy: we can't encode every possible combining sequence into a code point, so we might as well go the other way and decompose whenever possible. That's what Normalization Form D is about.

* In the meantime, we can use non-standard-library Rust code in order to deal with precomposed forms and decomposed forms.


### Reverse

Naively reversing a Unicode string can go wrong in several ways. 

* For example, merely reversing the chars (Unicode Scalar Values) in a string can cause combining marks to become attached to the wrong characters. 

* For example, suppose your string uses decomposed form and contains a combining sequence such a diaeresis. Naively reversing your string will tear the combining sequence, then apply the diaeresis to the wrong character.

Grapheme-level reversal may produce unexpected output.

* For example, if the input string contains certain non-printable control codes, such as directional formatting characters, then the output is unexpected.


## Requirements

This code is tested on Rust 1.5.3:

```sh
$ rustc --version                                                                     
rustc 1.53.0
```

To use this code, download it and run it as usual for Rust:

```sh
cargo run
```

Output:

```sh
Demo of noël with precomposed characters

String: noël
String length: 5
Characters debug: Chars(['n', 'o', 'ë', 'l'])
Characters count: 4
Characters reverse: lëon
Graphemes vector: ["n", "o", "ë", "l"]
Graphemes count: 4
Graphemes reverse: lëon

Demo of noël with decomposed characters

String: noël
String length: 6
Characters debug: Chars(['n', 'o', 'e', '\u{308}', 'l'])
Characters count: 5
Characters reverse: l̈eon
Graphemes vector: ["n", "o", "e\u{308}", "l"]
Graphemes count: 4
Graphemes reverse: lëon

Demo of mañana with precomposed characters

String: mañana
String length: 7
Characters debug: Chars(['m', 'a', 'ñ', 'a', 'n', 'a'])
Characters count: 6
Characters reverse: anañam
Graphemes vector: ["m", "a", "ñ", "a", "n", "a"]
Graphemes count: 6
Graphemes reverse: anañam

Demo of mañana with decomposed characters

String: mañana
String length: 8
Characters debug: Chars(['m', 'a', 'n', '\u{303}', 'a', 'n', 'a'])
Characters count: 7
Characters reverse: anãnam
Graphemes vector: ["m", "a", "n\u{303}", "a", "n", "a"]
Graphemes count: 6
Graphemes reverse: anañam

Demo of cats with precomposed characters

String: 😸😾
String length: 8
Characters debug: Chars(['😸', '😾'])
Characters count: 2
Characters reverse: 😾😸
Graphemes vector: ["😸", "😾"]
Graphemes count: 2
Graphemes reverse: 😾😸
```


## Tracking

* Package: demo-rust-string-issues
* Version: 2.0.0
* Updated: 2021-06-23T19:01:11Z
* License: BSD or GPL or MIT
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
