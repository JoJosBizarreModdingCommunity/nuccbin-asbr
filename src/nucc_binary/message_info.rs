use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[brw(pad_after = 0xC)]
    #[serde(with = "hex::serde")]
    pub crc32: [u8; 4],
   
    #[serde(skip)]
    pub text1_ptr: u64,

    #[serde(with = "hex::serde")]
    pub reference_crc32: [u8; 4],
    pub unk1: i16,

    #[brw(little)]
    pub acb_file_id: i16,

    pub acb_cue_id: i16,
    #[brw(pad_after = 0x4)]
    pub unk2: i16,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub text1: String,

  
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageInfo {

    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for MessageInfo {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::MessageInfo
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

impl From<&[u8]> for MessageInfo {
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
        .map(|(i, e)| (((0x28 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.text1 = read_string_from_ptr(&mut reader, entry.text1_ptr, current_offset + 0x10);

        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries
        }
    }
}

impl From<MessageInfo> for Vec<u8> {
    fn from(mut message_info: MessageInfo) -> Self {
        let mut writer = Cursor::new(Vec::new());

        message_info.entry_count = message_info.entries.len() as u32; // Update entry count

        writer.write_le(&1001u32).unwrap(); // Write the version

        writer.write_le(&message_info.entry_count).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&message_info.entries).unwrap();

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

        for (current_offset, entry) in message_info.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x28 * i + HEADER_SIZE) as u64, e)))
        {   
            write_ptr_to_string(&mut writer, &entry.text1, current_offset, 0x10);
            

        }


        writer.into_inner()
        
    }
}