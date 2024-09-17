use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const SFX_NAME_LEN: usize = 0x40;
const FILEPATH_LEN: usize = 0x80;
const ANM_NAME_LEN: usize = 0x40;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = SFX_NAME_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(SFX_NAME_LEN - x.len()).as_str()).into_bytes())]
    pub sfx_name_id: String,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = FILEPATH_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(FILEPATH_LEN - x.len()).as_str()).into_bytes())]
    pub filepath: String,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = ANM_NAME_LEN)]
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(ANM_NAME_LEN - x.len()).as_str()).into_bytes())]
    pub anm_name: String,

    #[brw(pad_after = 0x3C)]
    pub unk1: u32,
    
    pub unk2: f32,
    #[brw(pad_after = 0x150)]
    pub unk3: f32,

}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Gion {
    #[bw(calc = entries.len() as u32)]
    pub entry_count: u32,

    #[serde(skip)]
    pub version: u32,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Gion {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Gion
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