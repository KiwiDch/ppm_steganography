use std::str::FromStr;

fn main() {
    let mut encoder = stegano::encoder::Encoder::new(std::path::PathBuf::from_str("examples/kiwi.ppm").unwrap()).unwrap();
    encoder.try_update_message("Hidden message !").unwrap();
    encoder.encode_and_save(std::path::PathBuf::from_str("examples/kiwi_message.ppm").unwrap()).unwrap();
}