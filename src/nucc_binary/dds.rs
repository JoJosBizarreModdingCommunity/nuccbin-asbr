use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Dds {
    pub file: Vec<u8>
}

impl NuccBinaryParsed for Dds {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Dds
    }

    fn extension(&self) -> String {
        String::from(".dds")
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

impl From<&[u8]> for Dds {
    fn from(data: &[u8]) -> Self {
        Self {
            file: data.to_vec()
        }
    }
}

impl From<Dds> for Vec<u8> {
    fn from(dds: Dds) -> Self {
        dds.file
    }
}