# Nano ID

[![Build Status](https://travis-ci.org/nikolay-govorov/nanoid.svg?branch=master)](https://travis-ci.org/nikolay-govorov/nanoid)
[![Crates.io](https://img.shields.io/crates/l/rustc-serialize.svg)](https://github.com/nikolay-govorov/nanoid/blob/master/LICENSE)
[![Crates package version](https://img.shields.io/crates/v/nanoid.svg)](https://crates.io/crates/nanoid)

A tiny, secure, URL-friendly, unique string ID generator for Rust

```rust
extern crate nanoid;

fn main() {
   let id = nanoid::simple(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
}
```

**Safe.** It uses cryptographically strong random APIs
and guarantees a proper distribution of symbols.

**Small.** Only 179 bytes (minified and gzipped). No dependencies.
It uses [Size Limit] to control size.

**Compact.** It uses a larger alphabet than UUID (`A-Za-z0-9_~`)
and has a similar number of unique IDs in just 21 symbols instead of 36.

## Usage

### Simple

The main module uses URL-friendly symbols (`A-Za-z0-9_~`) and returns an ID
with 21 characters.

```rust
extern crate nanoid;

fn main() {
   let id = nanoid::simple(); //=> "Uakgb_J5m9g~0JDMbcJqLJ"
}
```

Symbols `-,.()` are not encoded in the URL. If used at the end of a link
they could be identified as a punctuation symbol.

### Custom length

If you want to reduce ID length (and increase collisions probability),
you can pass the length as an argument generate function:

```rust
extern crate nanoid;

fn main() {
   let id = nanoid::generate(10); //=> "IRFa~VaY2b"
}
```

### Custom Alphabet or Length

If you want to change the ID's alphabet or length
you can use the low-level `custom` module.

```rust
extern crate nanoid;

fn main() {
   let id = nanoid::custom(
       10, &['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f']
   ); //=> "4f90d13a42"
}
```

Alphabet must contain 256 symbols or less.
Otherwise, the generator will not be secure.

### Custom Random Bytes Generator

You can replace the default safe random generator using the `complex` module.
For instance, to use a seed-based generator.


```rust
extern crate nanoid;

fn randomBytes () -> u32 { /* ... */ }

fn main() {
    fn random (size: usize) -> Vec<u32> {
        let mut bytes: Vec<u32> = vec![0; size];

        for i in 0..size {
            bytes[i] = randomBytes();
        }

        bytes
    }
    
    nanoid::complex(10, &['a', 'b', 'c', 'd', 'e', 'f'], random); //=> "fbaefaadeb"
}
```


`random` function must accept the array size and return an vector
with random numbers.

If you want to use the same URL-friendly symbols with `format`,
you can get the default alphabet from the `url` module:

```rust
extern crate nanoid;

fn randomBytes () -> u32 { /* ... */ }

fn main() {
    nanoid::complex(10, nanoid::SAFE_ALPHABET, random); //=> "93ce_Ltuub"
}
```

## Other Programming Languages

* [JS](https://github.com/ai/nanoid)
* [C#](https://github.com/codeyu/nanoid-net)
* [Crystal](https://github.com/mamantoha/nanoid.cr)
* [Go](https://github.com/matoous/go-nanoid)
* [Elixir](https://github.com/railsmechanic/nanoid)
* [Java](https://github.com/aventrix/jnanoid)
* [PHP](https://github.com/hidehalo/nanoid-php)
* [Python](https://github.com/puyuan/py-nanoid)
* [Ruby](https://github.com/radeno/nanoid.rb)
