# Change Log
This project adheres to [Semantic Versioning](http://semver.org/).

## 0.5.0
* Bump `rand` to 0.9
* Add `rngs::thread_local` random source (#36)
* `format` now accepts any `FnMut(usize) -> Vec<u8>` random generator, enabling
  seeded and stateful RNGs (#32, #41). Non-capturing `fn(usize) -> Vec<u8>`
  callers continue to work unchanged.
* `nanoid!` macro size argument now accepts any expression, not only a single
  token (#28)
* Specialized fast path for alphabets whose size is a power of two (#35).
  Note: for seeded RNGs paired with a power-of-two alphabet (e.g. `SAFE`, the
  new `HEX_*` presets), the number of random bytes consumed per ID has changed
  — the output for a given seed will differ from 0.4.0.
* Add `alphabet::HEX_LOWERCASE` and `alphabet::HEX_UPPERCASE` presets (#39)
* Optional `smartstring` feature for small-string-optimized output (#29)
* Refreshed CI (GitHub Actions across OS matrix), drop Travis/AppVeyor
* Switched benchmarks to `criterion`

## 0.4.0
* merge #25, from @fundon: bump the rand#0.8
* merge #18, from @svenstaro
* merge #21, from @svenstaro
* merge #19, from @svenstaro
* merge #20, from @svenstaro
* merge #16, from @Exr0n
* merge #15, from @Exr0n
* merge #10, from @nbraud


## 0.3.0
* merge #3, from @TheIronBorn: various small improvements
* merge #4, from @delimitry: fix typo in function name
* Replace the `~` to `-` in alphabet
* Add the common macros
* Refactor structure. Remove pseudo-fast generator. Move format in `lib.rs`

## 0.2.0
* Added support for Windows
* Moved to system randomness generator

## 0.1.3
* Renamed the safe alphabet
* Added readme from rustdoc

## 0.1.2
* Updated the random number engine.
* Fixed bugs in documentation.

## 0.1.1
* Integrated performance tests
* Added example of custom random number generator.

## 0.1.0
* Initial release

## 0.0.1
* The start of the project.
