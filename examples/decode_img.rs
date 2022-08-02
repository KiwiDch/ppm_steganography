use std::str::FromStr;
fn main(){
    let decoder = steganography::decoder::Decoder::new(std::path::PathBuf::from_str("examples/kiwi_hidden_image.ppm").unwrap()).unwrap();
    decoder.decode().unwrap().unwrap().save_to_file(std::path::PathBuf::from_str("examples/hidden_image.ppm").unwrap()).unwrap();
}