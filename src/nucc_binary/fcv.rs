use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fcv {
    pub file: Vec<u8>
}

impl NuccBinaryParsed for Fcv {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Fcv
    }
    
    fn extension(&self) -> String {
        String::from(".fcv")
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


impl From<&[u8]> for Fcv {
    fn from(data: &[u8]) -> Self {
        Self {
            file: data.to_vec()
        }
    }
}

impl From<Fcv> for Vec<u8> {
    fn from(fcv: Fcv) -> Self {
        fcv.file
    }
}