# ppm_steganography
A small crate to hide data using 2 lsb of each bytes into a ppm image
## Usage
encoding image from ./examples/encode_img
```rust
use std::str::FromStr;

fn main() {
    let mut encoder = ppm_steganography::encoder::Encoder::new(std::path::PathBuf::from_str("examples/kiwi.ppm").unwrap()).unwrap();

    encoder.try_update_from_file(std::path::PathBuf::from_str("examples/to_hide.ppm").unwrap()).unwrap();

    encoder.encode_and_save(std::path::PathBuf::from_str("examples/kiwi_hidden_image.ppm").unwrap()).unwrap();
}
```
see ./examples for others

[Crates.io](https://crates.io/crates/ppm_steganography)
[Docs.rs](https://docs.rs/ppm_steganography/0.1.0/ppm_steganography/)

## Licence
[MIT Licence](https://spdx.org/licenses/MIT.html) or [Apache 2.0](https://spdx.org/licenses/Apache-2.0.html)