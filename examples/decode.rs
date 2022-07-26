use std::str::FromStr;

fn main(){
    let decoder = stegano::decoder::Decoder::new(std::path::PathBuf::from_str("examples/kiwi_message.ppm").unwrap()).unwrap();

    println!("{}", decoder.decode().unwrap().unwrap()); //Print "Hidden Message !"
}