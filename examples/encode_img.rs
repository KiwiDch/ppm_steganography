use std::str::FromStr;


fn main() {
    let mut encoder = ppm_steganography::encoder::Encoder::new(std::path::PathBuf::from_str("examples/kiwi.ppm").unwrap()).unwrap();

    encoder.try_update_from_file(std::path::PathBuf::from_str("examples/to_hide.ppm").unwrap()).unwrap();

    encoder.encode_and_save(std::path::PathBuf::from_str("examples/kiwi_hidden_image.ppm").unwrap()).unwrap();
}