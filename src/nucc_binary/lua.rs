use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lua {
    pub file: Vec<u8>
}

impl NuccBinaryParsed for Lua {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Lua
    }
    
    fn extension(&self) -> String {
        String::from(".lua")
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


impl From<&[u8]> for Lua {
    fn from(data: &[u8]) -> Self {
        Self {
            file: data.to_vec()
        }
    }
}

impl From<Lua> for Vec<u8> {
    fn from(lua: Lua) -> Self {
        lua.file
    }
}