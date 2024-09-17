use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Png {
    pub file: Vec<u8>
}

impl NuccBinaryParsed for Png {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Png
    }
    
    fn extension(&self) -> String {
        String::from(".png")
    }

    fn serialize(&self) -> Vec<u8> {
        self.file.clone()
    }

    fn deserialize(data: &[u8]) -> Self
        where
            Self: Sized,
        {   
            Self {
                file: data.to_vec()
            }
        }
}


impl From<&[u8]> for Png {
    fn from(data: &[u8]) -> Self {
        Self {
            file: data.to_vec()
        }
    }
}

impl From<Png> for Vec<u8> {
    fn from(png: Png) -> Self {
        png.file
    }
}