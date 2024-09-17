use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub index: u32,
    
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = 8)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(8 - x.len()).as_str()).into_bytes())]
    pub characode: String
}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Characode {
    #[bw(calc = entries.len() as u32)]
    pub entry_count: u32,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Characode {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Characode
    }

    fn extension(&self) -> String {
        String::from(".json")
    }

    fn serialize(&self) -> Vec<u8> {
        serde_json::to_string_pretty(self).unwrap().into()
    }

    fn deserialize(data: &[u8]) -> Self
        where
            Self: Sized,

        {   
            serde_json::from_slice(data).unwrap()
        }
}