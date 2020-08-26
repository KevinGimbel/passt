# passt
> The "good-enough" password generator ¯\\_(ツ)_/¯

!["Passt logo"](.github/passt-logo-1.png)

`passt` is a zero-dependency random string generator that can be used to generate passwords in terminals or in your application.

<!-- BEGIN mktoc -->
- [Zero Dependencies?](#zero-dependencies)
- [Supported operating systems](#supported-operating-systems)
  - [*nix](#nix)
  - [Windows Support](#windows-support)
- [Usage: library](#usage-library)
- [Usage: cli](#usage-cli)
- [Known issues](#known-issues)
  - [Emoji support](#emoji-support)
- [Why the name "passt"](#why-the-name-passt)
- [License](#license)
<!-- END mktoc -->

## Zero Dependencies?

`passt` only depends on Rust standard library, namely:
- `std::fs::File`
- `use std::io::Read`

and additionally for the CLI part:
- `std::env`

and no other crates.

The only other dependency is `/dev/urandom` from which random ints are read to generate random values.

## Supported operating systems

### *nix

All GNU/Linux / *nix systems should be supported as long as they have `/dev/urandom`. Only tested on MacOS and Ubuntu.

### Windows Support

For Windows `file:/dev/urandom` is read but this is **not yet tested**. It may or may not work. 🤷‍♀️ Help with Windows support is appreciated!


## Usage: library

Use the standard character sets. This means possible characters are:
-  `a-zA-Z0-9` if no special chars are included
-  `a-zA-Z0-9` + `!§$%&/\\()=?´`-_.,:;#'+*<>°^` is special chars are included

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

```bash
passt -l <int> [-s]
```

```bash
$ passt -l 16 -s
D#§2§RgI0Ou°F#

$ passt -l 32
OgHFnTrSH5liCPhkrfbHdfhSWFwGGAPA
```

## Known issues

### Emoji support

Right now the generation does not work with a string of emojis. This must have to do with the way the random characters are extracted.

## Why the name "passt"

"passt" is a German word you can say if something is "okay". Since this tool is "okay" in generating strings that can be used for passwords I found the name fitting. 

## License

`passt` is primarily distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.