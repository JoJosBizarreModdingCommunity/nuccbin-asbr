use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Xml {
    pub file: Vec<u8>
}

impl NuccBinaryParsed for Xml {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Xml
    }
    
    fn extension(&self) -> String {
        String::from(".xml")
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


impl From<&[u8]> for Xml {
    fn from(data: &[u8]) -> Self {
        Self {
            file: data.to_vec()
        }
    }
}

impl From<Xml> for Vec<u8> {
    fn from(xml: Xml) -> Self {
        xml.file
    }
}