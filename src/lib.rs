use error::SteganoError;
use image::GenericImageView;

pub mod error {
    #[derive(Debug)]
    pub enum SteganoError {
        MessageTooLong,
        BadFormat,
        ImpossibleToParse,
        FileError(std::io::Error),
        ImageError(image::error::ImageError)
    }

    impl From<std::io::Error> for SteganoError {
        fn from(e : std::io::Error) -> Self {
            SteganoError::FileError(e)
        }
    }

    impl From<image::error::ImageError> for SteganoError {
        fn from(e : image::error::ImageError) -> Self {
            SteganoError::ImageError(e)
        }
    }

    impl std::fmt::Display for SteganoError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                SteganoError::MessageTooLong => write!(f,"Message is too long"),
                SteganoError::BadFormat => write!(f,"Incompatible format(Only .ppm image)"),
                SteganoError::ImpossibleToParse =>  write!(f,"Impossible to parse the hidden message"),
                SteganoError::FileError(e) => e.fmt(f),
                SteganoError::ImageError(e) => e.fmt(f),
            }
        }
    }

    impl std::error::Error for SteganoError {}
}

pub mod encoder {
    //! Encoder write data on the 2 lsb on each bytes of the image given
    use std::io::Read;

    use crate::error::SteganoError;
    use crate::{open_image_ppm_only,image_to_vec_rgb};

    use image::DynamicImage;
    pub struct Encoder {
        image: DynamicImage,
        buf: Vec<u8>
    }

    impl Encoder {


        ///Create a new [`Encoder`] for steganography
        ///# Error
        ///May return [`crate::error::SteganoError::BadFormat`] or [`crate::error::SteganoError::ImageError`]
        pub fn new(path: std::path::PathBuf) -> Result<Self,SteganoError> {
            let image = open_image_ppm_only(path)?;
            
            Ok(Encoder { 
                image,
                buf: Vec::new(),
            })
        }

        ///Try to update the hidden message from &str
        ///# Error
        ///Same error than [`Encoder::try_update_from_bytes`]
        pub fn try_update_message(&mut self, message: &str) -> Result<(), SteganoError>{
            self.try_update_from_bytes(message.as_bytes())?;
            Ok(())
        }

         ///Try to update the hidden message from bytes arrays
        ///# Error
        ///May return [`crate::error::SteganoError::MessageTooLong`] if the image doesn't contains enough bytes to write the message
        pub fn try_update_from_bytes(&mut self,message: &[u8])-> Result<(),SteganoError> {
            let message = [message,b"STO"].concat();


            if  message.len() * 4 > 3 * (self.image.width()*self.image.height()) as usize {
                return Err(SteganoError::MessageTooLong);
            }

            self.buf = message;

            Ok(())
        }


        ///Try to update the hidden message from file path
        ///# Error
        ///Same error than [`Encoder::try_update_from_bytes`] and [`crate::error::SteganoError::FileError`]
        pub fn try_update_from_file(&mut self, path: std::path::PathBuf) -> Result<(),SteganoError> {
            let msg = std::fs::File::open(path)?;
            let mut reader = std::io::BufReader::new(msg);
            let mut buf = Vec::new();
            reader.read_to_end(&mut buf)?;
            self.try_update_from_bytes(&buf)?;
            Ok(())
        }


        //Stegonography format: data/STO(u8 *3).
        ///Encode the message bytes on 2 bits/bytes of the original image and save the result at the given path
        /// # Error
        /// May be [`crate::error::SteganoError::FileError`]
        pub fn encode_and_save(self, output_path: std::path::PathBuf) -> Result<(), SteganoError>{

            let mut output_img= image::ImageBuffer::new(self.image.width(), self.image.height());
            let mut image_bytes_composante:Vec<u8> = image_to_vec_rgb(&self.image);
        
            for (i, composant) in image_bytes_composante[..self.buf.len() * 4].iter_mut().enumerate() { //correspond au nombre d'octet que va prendre un message codé sur 2 lsb (8/2)
                if (i as f32 * (2f32/8f32)) as usize >= self.buf.len(){
                    break;
                }
                *composant = (*composant & 0b11111100) + (self.buf[(i as f32 * (2f32/8f32)) as usize] >> (8 - ((i % 4) * 2 + 2)) & 0b00000011);
            }
        
            let mut new_img_data = image_bytes_composante.into_iter();
        
            for (_,_,pixel) in output_img.enumerate_pixels_mut() {
                *pixel = image::Rgb([new_img_data.next().unwrap(),new_img_data.next().unwrap(),new_img_data.next().unwrap()]);
            }
        
            output_img.save(output_path)?;

            Ok(())
        }
    }
}

pub mod decoder {
    //! Decoder try to find data written with the same format of the Encoder
    use std::io::Write;
    use image::DynamicImage;
    use crate::{open_image_ppm_only,image_to_vec_rgb};
    use crate::error::SteganoError;


    pub struct Data {
        data: Vec<u8>
    }
    
    impl Data {
        pub fn try_as_text(self) -> Result<String, SteganoError> {
            let contenu = std::string::String::from_utf8(self.data[..self.data.len()-3].to_vec());
            if contenu.is_err() {
                Err(SteganoError::ImpossibleToParse)
            }
            else {
                Ok(contenu.unwrap())
            }
        }

        pub fn data(&self) -> &Vec<u8> {
            &self.data
        }

        pub fn save_to_file(self,path: std::path::PathBuf) -> Result<(),SteganoError> {
            let file = std::fs::File::create(path)?;
            let mut writer = std::io::BufWriter::new(file);
            writer.write_all(&self.data[..self.data.len()-3])?;
            Ok(())
        }
    }

    pub struct Decoder {
        image: DynamicImage,
    }

    impl Decoder {
        pub fn new(path: std::path::PathBuf) -> Result<Self, SteganoError> {
            let image = open_image_ppm_only(path)?;
            Ok(
                Decoder {
                    image
                }
            )
        }

        pub fn decode(self) -> Result<Option<Data>,SteganoError> {
            let bytes_img = image_to_vec_rgb(&self.image);
            let bytes_parsed = self.parse(&bytes_img[..]);
            
            if bytes_parsed.is_err() {
                return Ok(None);

            }

            let bytes_parsed = bytes_parsed.unwrap();

            let data = Data {
                data: bytes_parsed,
            };

            Ok(Some(data))
        }

        fn parse(&self, bytes: &[u8]) -> Result<Vec<u8>,SteganoError> {
        let mut v_lsb:Vec<u8> = Vec::new();
        let mut temp = 0u8;
        let mut stop = None;

        for (i,&e) in bytes.iter().enumerate() {

            if i % 4 == 0 && i !=0 {
                v_lsb.push(temp);
                temp = 0u8;
            }
            if v_lsb.len() >= 3 && &v_lsb[v_lsb.len()-3..v_lsb.len()] == b"STO" {
                stop = Some(i);
                break;
            }
            temp = (temp << 2) + (e & 0b00000011);
        }

        if stop.is_some() {
            Ok(v_lsb)
        }
        else {
            Err(SteganoError::ImpossibleToParse)
        }
        
        }
        
    }

}


fn open_image_ppm_only(path: std::path::PathBuf) -> Result<image::DynamicImage, SteganoError> {

    let decoder = image::codecs::pnm::PnmDecoder::new(std::io::BufReader::new(std::fs::File::open(path)?))?;

    if let image::codecs::pnm::PnmSubtype::Pixmap(_) = decoder.subtype() {
        let image = image::DynamicImage::from_decoder(decoder)?;
        Ok(image)
    }
    else {
        return Err(error::SteganoError::BadFormat.into());
    }
}

fn image_to_vec_rgb(image: &image::DynamicImage) -> Vec<u8> {
    let mut image_bytes_composante:Vec<u8> = Vec::new();

    for pixel in image.pixels() {
        for composant in &pixel.2.0[..=2]{ //remove alpha
            image_bytes_composante.push(*composant);
        }
    }

    image_bytes_composante
}