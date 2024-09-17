use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


use super::HEADER_SIZE;

// Format was reversed by https://github.com/al-hydra
#[allow(non_snake_case)]
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub characode_ptr: u64,

    pub index: u32,

    pub ofsX: f32,
    pub ofsY: f32,

    pub unk1: f32,
    pub scale: f32,
    pub unk2: f32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub characode: String,
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomizeDefaultParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for CustomizeDefaultParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::CustomizeDefaultParam
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

impl From<&[u8]> for CustomizeDefaultParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        
        let version = reader.read_le::<u32>().unwrap();
        let entry_count = reader.read_le::<u32>().unwrap();
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
            .map(|(i, e)| (((0x20 * i + HEADER_SIZE) as u64, e))) 
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

impl From<CustomizeDefaultParam> for Vec<u8> {
    fn from(mut customize_default_param: CustomizeDefaultParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        customize_default_param.entry_count = customize_default_param.entries.len() as u32; // Update entry count


        writer.write_le(&1000u32).unwrap(); // Write the version
        writer.write_le(&customize_default_param.entry_count).unwrap();
        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&customize_default_param.entries).unwrap();

        for (current_offset, entry) in customize_default_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x20 * i + HEADER_SIZE) as u64, e)))
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