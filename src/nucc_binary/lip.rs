use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x10;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub timing: f32,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub anm_name: String,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub type_name: String,

    pub unk2: f32,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Lip {
    #[bw(calc = self.entries.len() as u16)]
    pub entry_count: u16,



    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Lip {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Lip
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
