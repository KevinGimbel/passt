# passt
> The "good-enough" password generator ¯\\_(ツ)_/¯

!["Passt logo"](.github/passt-logo-1.png)

`passt` is a "zero-dependency" random string generator that can be used to generate passwords in terminals or in your application.

<!-- BEGIN mktoc -->
- [Zero Dependencies?](#zero-dependencies)
- [Supported operating systems](#supported-operating-systems)
  - [*nix](#nix)
  - [Windows Support](#windows-support)
- [Usage: library](#usage-library)
- [Usage: cli](#usage-cli)
  - [Install](#install)
- [Limitations](#limitations)
- [Why the name "passt"](#why-the-name-passt)
- [License](#license)
<!-- END mktoc -->

---

## Zero Dependencies?

`passt` only depends on Rust standard library, namely:
- `std::fs::File`
- `std::io::Read`

and additionally for the CLI part:
- `std::env`
- `std::process::exit`

and no other crates.

The only other "dependency" is `/dev/urandom` from which random ints are read to generate random values. So "zero-dependency" may be a bit of a stretch. 😬

## Supported operating systems

### *nix

All GNU/Linux / *nix systems should be supported as long as they have `/dev/urandom`. Only tested on MacOS and Ubuntu.

### Windows Support

For Windows `file:/dev/urandom` is read but this is **not yet tested**. It may or may not work. 🤷‍♀️ Help with Windows support is appreciated!


## Usage: library

**Using the standard character set**

This means possible characters are:
-  `a-zA-Z0-9` if no special chars are included
-  `a-zA-Z0-9` and `!§$%&/()=?´-_.,:;#'+*<>°^` if special chars are included

```rust
use passt:Passt;

fn my_random_password() -> String {
    // Passt::random_password(length: i32, with_special_chars: Option<bool>) -> String {
    Passt::random_password(16, Some(false));
}

fn my_random_password_with_none() -> String {
    // Passt::random_password(length: i32, with_special_chars: Option<bool>) -> String {
    Passt::random_password(16, None);
}

fn my_random_password_with_special_chars() -> String {
    Passt::random_password(16, Some(true));
}
```

**Specify custom character set**

This allows you to use a different set of possible characters.

```rust
fn my_custom_set() {
    // Create password only from random chars "acefhjlnprtvxz13579"
    Pass::random_password_with_custom_set(16, "acefhjlnprtvxz13579")
}
```

## Usage: cli

### Install

Install with `cargo`:

```bash
cargo install passt
```

Then use as described below

```bash
USAGE: passt -l <int> [-s] [-chars "<str>"]

-l      length of the generated password
-s      use special characters
-chars  possible characters as a string, e.g. "abc012"
```

```bash
$ passt -l 16 -s
D#§2§RgI0Ou°F#

$ passt -l 32
OgHFnTrSH5liCPhkrfbHdfhSWFwGGAPA

# Custom set with emojis!
$ passt -l 16 -chars "🛹🥗🌈🦔🕶🤳🎮"
🌈🎮🎮🎮🤳🥗🎮🌈🎮🎮🎮🎮🤳🎮🕶🕶

$ passt -l 4 -chars "01"
```

## Limitations

Because the random extraction of characters is weak it is better to have duplicates in the character set. See the following example:

```bash
passt -l 4 -chars "10"
0000
```

With two characters, the last char is always taken. For randomness, add more chars to the set.

```bash
passt -l 4 -chars "1010"
0100
```

## Why the name "passt"

"passt" is a German word you can say if something is "okay". Since this tool is "okay" in generating random strings that can be used for passwords I found the name fitting.

## License

`passt` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.