use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};


use super::{NuccBinaryParsed, NuccBinaryType};

use super::HEADER_SIZE;

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub card_ptr: u64,

    pub part: u32,
    pub interaction_type: u32,
    #[brw(pad_after = 4)]
    pub medal_type: u32,


    #[serde(skip)]
    pub letter_ptr: u64,

    pub unk1: i32,
    pub unk2: i32,
    pub unk3: i32,
    pub unk4: i32,

    #[serde(skip)]
    pub sfx1_ptr: u64,
    #[serde(skip)]
    pub sfx2_ptr: u64,
    #[serde(skip)]
    pub sfx3_ptr: u64,
    #[serde(skip)]
    pub sfx4_ptr: u64,

    pub unk5: u32,

    #[brw(pad_before = 4)]
    #[serde(skip)]
    pub characode_ptr: u64,

    pub dlc_no: u32,
    pub patch_no: u32,
    pub unlock_condition: u32,
    pub unk6: u32,
    pub cost: u32,
    
    #[brw(pad_before = 12)]
    #[serde(skip)]
    pub card_detail_ptr: u64,

    #[brw(pad_after = 4)]
    pub index: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub card_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub letter: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx1: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx2: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx3: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub sfx4: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub characode: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub card_detail: String
} 

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct CustomCardParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,

    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}


impl NuccBinaryParsed for CustomCardParam {

    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::CustomCardParam
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


impl From<&[u8]> for CustomCardParam {
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
            .map(|(i, e)| (((0x90 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.card_id = read_string_from_ptr(&mut reader, entry.card_ptr, current_offset + 0x0);
            entry.letter = read_string_from_ptr(&mut reader, entry.letter_ptr, current_offset + 0x18);
            entry.sfx1 = read_string_from_ptr(&mut reader, entry.sfx1_ptr, current_offset + 0x30);
            entry.sfx2 = read_string_from_ptr(&mut reader, entry.sfx2_ptr, current_offset + 0x38);
            entry.sfx3 = read_string_from_ptr(&mut reader, entry.sfx3_ptr, current_offset + 0x40);
            entry.sfx4 = read_string_from_ptr(&mut reader, entry.sfx4_ptr, current_offset + 0x48);
            entry.characode = read_string_from_ptr(&mut reader, entry.characode_ptr, current_offset + 0x58);
            entry.card_detail = read_string_from_ptr(&mut reader, entry.card_detail_ptr, current_offset + 0x80);
        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries
        }
    }
}


impl From<CustomCardParam> for Vec<u8> {
    fn from(mut custom_card_param: CustomCardParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        custom_card_param.entry_count = custom_card_param.entries.len() as u32; // Update entry count

        writer.write_le(&1000u32).unwrap(); // Write the version

        writer.write_le(&custom_card_param.entry_count).unwrap();

        writer.write_le(&8u64).unwrap(); // Write the entry ptr offset (always 8)

        writer.write_le(&custom_card_param.entries).unwrap();

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
        for (current_offset, entry) in custom_card_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x90 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.card_id, current_offset, 0);
            write_ptr_to_string(&mut writer, &entry.letter, current_offset, 0x18);
            write_ptr_to_string(&mut writer, &entry.sfx1, current_offset, 0x30);
            write_ptr_to_string(&mut writer, &entry.sfx2, current_offset, 0x38);
            write_ptr_to_string(&mut writer, &entry.sfx3, current_offset, 0x40);
            write_ptr_to_string(&mut writer, &entry.sfx4, current_offset, 0x48);
            write_ptr_to_string(&mut writer, &entry.characode, current_offset, 0x58);
            write_ptr_to_string(&mut writer, &entry.card_detail, current_offset, 0x80);
        }


        writer.into_inner()
    }

}
