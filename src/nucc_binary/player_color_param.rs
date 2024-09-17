use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek,SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

// Format was reversed by https://github.com/SutandoTsukai181
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub characode_ptr: u64,

    pub costume_index: u32,

    #[serde(with = "hex::serde")] // Serialize as hex string
    #[br(map = |x: [u32; 3]| { x.into_iter().map(|e| e as u8).collect() })] // Read as u32, map to u8 and collect into Vec<u8>
    #[bw(map = |x: &Vec<u8>| { x.into_iter().map(|e| *e as u32).collect::<Vec<u32>>() })] // Read as Vec<u8>, map to u32 and collect into Vec<u32>
    pub color_code: Vec<u8>,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub characode: String,
}


#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerColorParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u16,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for PlayerColorParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::PlayerColorParam
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


impl From<&[u8]> for PlayerColorParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
       
        let version = reader.read_le::<u32>().unwrap();
        let entry_count = reader.read_le::<u16>().unwrap();
        let entry_ptr = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we have enough space to avoid reallocations

        for _ in 0..entry_count as usize {
            let entry = reader.read_le::<Entry>().unwrap();
            entries.push(entry);
        }

        for (current_offset, entry) in entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x18 * i + HEADER_SIZE) as u64, e))) 
        {
            if entry.characode_ptr != 0 {
                reader.seek(SeekFrom::Start(current_offset as u64)).unwrap();
                reader.seek(SeekFrom::Current(entry.characode_ptr as i64)).unwrap();
                entry.characode = reader.read_be::<NullString>().unwrap().to_string();
            } else {
                entry.characode = String::from("");
            }
        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries
        }
    }
}


impl From<PlayerColorParam> for Vec<u8> {
    fn from(mut player_color_param: PlayerColorParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        player_color_param.entry_count = player_color_param.entries.len() as u16; // Update entry count

        writer.write_le(&1000u32).unwrap(); // Write the version as 1000
        writer.write_le(&player_color_param.entry_count).unwrap();        
        writer.write_le(&8u64).unwrap(); // Write the entry ptr offset (always 8)

        writer.write_le(&player_color_param.entries).unwrap();

        for (current_offset, entry) in player_color_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x18 * i + HEADER_SIZE) as u64, e)))
        {
            if !entry.characode.is_empty() {
                writer.seek(SeekFrom::Start(current_offset as u64)).unwrap();
                let string_pos = writer.seek(SeekFrom::End(0)).unwrap();
                writer.write_be::<NullString>(&NullString::from(entry.characode.clone())).unwrap();

                // Align to 8 bytes
                let pos = writer.seek(SeekFrom::Current(0)).unwrap() - string_pos;
                if 8 - (pos % 8) != 8  {
                    writer.write_le::<Vec<u8>>(&vec![0; 8 - (pos % 8) as usize]).unwrap();
                }

                writer.seek(SeekFrom::Start(current_offset as u64)).unwrap();
                writer.write_le::<u64>(&(string_pos - current_offset)).unwrap();
            }
        }

        writer.into_inner()

        
    }
}