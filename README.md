# Demonstration of Rust string issues

Demonstration of Rust string issues,
such as UTF-8 encoding, accents, etc.

Feedback welcome.

This repo is based on the artice about string handling here:

* https://mortoray.com/2013/11/27/the-string-type-is-broken/

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
Does it print correctly? noël
What is the character length? 4
What is the reverse? lëon
What are the first three characters? noë
Does it print correctly? 😸😾
What is the character length? 2
What is the reverse? 😾😸
What is the substring after the first character? 😾
```

Tracking:

* Package: demo-rust-string-issues
* Version: 1.1.0
* Updated: 2021-06-23T02:42:18Z
* License: BSD or GPL or MIT
* Contact: Joel Parker Henderson <joel@joelparkerhenderson.com>
