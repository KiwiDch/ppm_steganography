use std::str::FromStr;

fn main(){
    let decoder = ppm_steganography::decoder::Decoder::new(std::path::PathBuf::from_str("examples/kiwi_message.ppm").unwrap()).unwrap();

    println!("{}", decoder.decode().unwrap().unwrap().try_as_text().unwrap()); //Print "Hidden Message !"
}