use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    pub interaction_type: u32,
    pub round_win_animation: u32,

    #[serde(skip)]
    pub text1_ptr: u64,

    #[serde(skip)]
    pub text2_ptr: u64,

    #[serde(skip)]
    pub text3_ptr: u64,

    #[serde(skip)]
    pub text4_ptr: u64,


    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub text1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub text2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub text3: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub text4: String 
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct SpeakingLineParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for SpeakingLineParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::SpeakingLineParam
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

impl From<&[u8]> for SpeakingLineParam {
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

        fn read_string_from_ptr(reader: &mut Cursor<&[u8]>, ptr: u64, curent_offset: u64) -> String {
            if ptr != 0 {
                reader.seek(SeekFrom::Start(curent_offset as u64)).unwrap();
                reader.seek(SeekFrom::Current(ptr as i64)).unwrap();
                reader.read_be::<NullString>().unwrap().to_string()
            } else {
                String::from("")
            }
        }

        for (current_offset, entry) in entries
        .iter_mut()
        .enumerate()
        .map(|(i, e)| (((0x28 * i + HEADER_SIZE) as u64, e))) {
            entry.text1 = read_string_from_ptr(&mut reader, entry.text1_ptr, current_offset + 0x8);
            entry.text2 = read_string_from_ptr(&mut reader, entry.text2_ptr, current_offset + 0x10);
            entry.text3 = read_string_from_ptr(&mut reader, entry.text3_ptr, current_offset + 0x18);
            entry.text4 = read_string_from_ptr(&mut reader, entry.text4_ptr, current_offset + 0x20);
        }

        SpeakingLineParam {
            version,
            entry_count,
            entry_ptr,
            entries
        }
    }
}


impl From<SpeakingLineParam> for Vec<u8> {
    fn from(mut speaking_line_param: SpeakingLineParam) -> Self {
        let mut writer = Cursor::new(Vec::new());

        speaking_line_param.entry_count = speaking_line_param.entries.len() as u32; // Update entry count

        writer.write_le(&1000u32).unwrap();

        writer.write_le(&speaking_line_param.entry_count).unwrap();
        writer.write_le(&8u64).unwrap();

        writer.write_le(&speaking_line_param.entries).unwrap();

        fn write_ptr_to_string(
            writer: &mut Cursor<Vec<u8>>,
            string: &String,
            current_offset: u64,
            adjustment: u64,
        ) {
            if !string.is_empty() {
               writer.seek(SeekFrom::End(0)).unwrap();
                let string_pos = writer.seek(SeekFrom::End(0)).unwrap();
                writer.write_be::<NullString>(&NullString::from(string.clone())).unwrap();

                // Align to 8 bytes
                let pos = writer.seek(SeekFrom::Current(0)).unwrap() - string_pos;
                if 8 - (pos % 8) != 8  {
                    writer.write_le::<Vec<u8>>(&vec![0; 8 - (pos % 8) as usize]).unwrap();
                }

                writer.seek(SeekFrom::Start((current_offset + adjustment) as u64)).unwrap();
                writer.write_le::<u64>(&(string_pos - current_offset - &adjustment)).unwrap();
                
            }
        }

        for (current_offset, entry) in speaking_line_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x28 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.text1, current_offset, 0x8);
            write_ptr_to_string(&mut writer, &entry.text2, current_offset, 0x10);
            write_ptr_to_string(&mut writer, &entry.text3, current_offset, 0x18);
            write_ptr_to_string(&mut writer, &entry.text4, current_offset, 0x20);
        }


        writer.into_inner()

        
    }
}