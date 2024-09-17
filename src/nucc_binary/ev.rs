use binrw::binrw;
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};

const STR_LEN: usize = 0x20;

// Format was reversed by https://github.com/TheLeonX
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[br(map = |x: Vec<u8>| Entry::decrypt(x), count = STR_LEN)]
    #[bw(map = |x: &String| Entry::encrypt(x, STR_LEN))]
    pub sound_name: String,

    pub unk0: i16,
    pub volume: f32,

    pub pitch: i16,
    pub unk2: i16,

    pub unk3: i16,
    pub timing: i16,

    pub unk4: f32,

    pub unk5: f32,

    #[br(map = |x: Vec<u8>| Entry::decrypt(x), count = STR_LEN)]
    #[bw(map = |x: &String| Entry::encrypt(x, STR_LEN))]
    pub anm_path: String,


    #[br(map = |x: Vec<u8>| Entry::decrypt(x), count = STR_LEN)]
    #[bw(map = |x: &String| Entry::encrypt(x, STR_LEN))]
    pub anm_name: String,


    #[br(map = |x: Vec<u8>| Entry::decrypt(x), count = STR_LEN)]
    #[bw(map = |x: &String| Entry::encrypt(x, STR_LEN))]
    pub bone: String,


    pub unk6: u32,
    pub unk7: u32,
    pub unk8: u32,

    pub unk9: u32,

    pub unk10: u32,

    #[br(map = |x: Vec<u8>| Entry::decrypt(x), count = STR_LEN)]
    #[bw(map = |x: &String| Entry::encrypt(x, STR_LEN))]
    pub pl_anm: String,
    
}

// Decryption/encryption algorithm by https://github.com/SutandoTsukai181
impl Entry {
    fn xor(data: &[u8]) -> Vec<u8> {
        let mut key = b"\x8C\x91\x9B\x9A\x89\xD1\x87\x99\x9D\x96\x91"
            .iter()
            .cycle();

        let mut block = vec![];
        let mut result = vec![];

      
        for byte in data {
            block.push(byte ^ key.next().unwrap());

            if block.len() == 4 {
                result.extend(block.into_iter().rev());
                block = vec![];
            }
        }
    
        result
        
    }

    fn decrypt(mut data: Vec<u8>) -> String {
        data = Self::xor(&data);
        String::from_utf8_lossy(&data.into_iter().take_while(|b| *b != 0).collect::<Vec<u8>>()).to_string()
    }

    fn encrypt(string: &String, len: usize) -> Vec<u8> {
        let string = &String::from("\0").repeat(len - string.len());
        let mut data = string.clone().into_bytes();
        data = Entry::xor(&data);
        data

        
    }

}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Ev {
    #[bw(calc = self.entries.len() as u16)]
    pub entry_count: u16,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for Ev {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::Ev
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