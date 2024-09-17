use binrw::binrw;
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x20;


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub folder: String,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(STR_LEN - x.len()).as_str()).into_bytes())]
    pub code: String,

    #[brw(pad_after = 4)]
    pub file_type: u32,

}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct PrmLoad {
    #[bw(calc = entries.len() as u32)]
    pub entry_count: u32,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}


impl NuccBinaryParsed for PrmLoad {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::PrmLoad
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