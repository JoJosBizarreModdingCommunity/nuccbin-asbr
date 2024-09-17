/*use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::NuccChunkBinaryParsed;

const HEADER_SIZE: usize = 0x14; // Size of the NUCC Binary header

// Format was reversed by https://github.com/al-hydra
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub part: u32,
    pub unk1: u32,

    #[serde(skip)]
    pub panel_string1_ptr: u64,

    pub unk2: u32,
    pub unk3: u32,

    #[serde(skip)]
    pub panel_string2_ptr: u64,

    #[serde(skip)]
    pub panel_string3_ptr: u64,

    #[serde(skip)]
    pub panel_string4_ptr: u64,

    pub test1: u32,
    pub test2: u32,

    #[serde(skip)]
    pub panel_string5_ptr: u64,

    #[serde(skip)]
    pub panel_string6_ptr: u64,

    #[serde(skip)]
    pub panel_string7_ptr: u64,

    #[serde(skip)]
    pub panel_string8_ptr: u64,

    pub test3: u32,
    pub test4: u32,

    #[serde(skip)]
    pub panel_string9_ptr: u64,

    pub test5: u32,
    pub test6: u32,

    #[serde(skip)]
    pub panel_string10_ptr: u64,


    #[serde(skip)]
    pub panel_string11_ptr: u64,

    #[serde(skip)]
    pub panel_string12_ptr: u64,

    #[serde(skip)]
    pub panel_string13_ptr: u64,

    #[serde(skip)]
    pub panel_string14_ptr: u64,

    #[serde(skip)]
    pub panel_string15_ptr: u64,

    #[serde(skip)]
    pub panel_string16_ptr: u64,

    #[serde(skip)]
    pub panel_string17_ptr: u64,

    #[serde(skip)]
    pub panel_string18_ptr: u64,

    #[serde(skip)]
    pub panel_string19_ptr: u64,

    #[serde(skip)]
    pub panel_string20_ptr: u64,

    pub unk4: i32,
    pub unk5: i32,
    pub unk6: i32,
    pub unk7: i32,

    #[serde(skip)]
    pub panel_string21_ptr: u64,


    pub unk8: u32,

    pub gold: u32,

    pub unk9: u32,
    pub unk10: u32,
    pub unk11: u32,
    pub unk12: u32,

    #[serde(skip)]
    pub reward_ptr: u64,

    pub unk13: u32,
    pub unk14: u32,
    pub unk15: u32,
    pub unk16: u32,

    #[serde(skip)]
    pub card_id_ptr: u64,

    pub unk17: u32,
    pub unk18: u32,
    pub unk19: i32,
    pub unk20: u32,
    pub unk21: u32,
    pub unk22: u32,
    pub unk23: u32,
    pub unk24: u32,
    pub unk25: u32,
    pub unk26: u32,
    pub unk27: u32,
    pub unk28: u32,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string3: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string4: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string5: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string6: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string7: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string8: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string9: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string10: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string11: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string12: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string13: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string14: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string15: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string16: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string17: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string18: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string19: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string20: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub panel_string21: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub reward: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub card_id: String
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct MainModeParam {
    pub size: u32,
    pub version: u32,

    pub entry_count: u16,
    pub unk0: u16,

    pub unk1: u32,

    #[brw(pad_before = 4)]
    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccChunkBinaryParsed for MainModeParam {
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
}*/


