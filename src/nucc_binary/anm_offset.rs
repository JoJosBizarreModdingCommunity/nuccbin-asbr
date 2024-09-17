use binrw::binrw;
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

const ANM_STR_LEN: usize = 0x40;
const CHARACODE_LEN: usize = 0x10;

#[allow(non_snake_case)]
#[binrw]
#[brw(little)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry { // param entry, looks like each entry is 0x24 bytes? sometimes 0x20?
    pub frame_min: u32,
    pub frame_max: u32,

    pub ofsMinX: f32,
    pub ofsMinY: f32,
    pub ofsMinZ: f32,

    pub ofsMaxX: f32,
    pub ofsMaxY: f32,
    pub ofsMaxZ: f32,

    pub ofs_type: u32
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Anmofs {
    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = ANM_STR_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(ANM_STR_LEN - x.len()).as_str()).into_bytes())]
    pub anm_name: String,

    #[br(map = |x: Vec<u8>| String::from_utf8_lossy(&x).trim_end_matches('\u{0}').to_string(), count = CHARACODE_LEN)] // Need to trim the null bytes
    #[bw(map = |x: &String| (x.clone() + String::from('\u{0}').repeat(CHARACODE_LEN - x.len()).as_str()).into_bytes())]
    pub characode: String, 

    pub entry_count: u32,

    #[br(count = entry_count)]
    #[brw(pad_before = 0x4)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Anmofs {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Anmofs
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


