use binrw::{binrw, BinReaderExt, BinWriterExt, NullString};
use binrw::io::{Cursor, Seek, SeekFrom};
use serde::{Serialize, Deserialize};

use super::{NuccBinaryParsed, NuccBinaryType};


use super::HEADER_SIZE;

// Format was reversed by https://github.com/al-hydra
#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    #[serde(skip)]
    pub art_id_ptr: u64,

    #[serde(skip)]
    pub icon_path_ptr: u64,

    #[serde(skip)]
    pub icon_filename_ptr: u64,

    #[serde(skip)]
    pub img_path_ptr: u64,

    #[serde(skip)]
    pub img_filename_ptr: u64,

    pub part: u32,
    pub unk0: u32,

    #[serde(skip)]
    pub characode_ptr: u64,

    pub dlc_no: u32,
    pub patch_no: u32,

    pub unlock_condition: u32,
    pub unk3: u32,

    pub cost: u32,
    pub unk4: u32,

    #[serde(skip)]
    pub art_name_ptr: u64,

    #[serde(skip)]
    pub art_desc_ptr: u64,

    #[brw(pad_after = 4)]
    pub art_num: u32,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub art_id: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub icon_path: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub icon_filename: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub img_path: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub img_filename: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub characode: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub art_name: String,

    #[brw(ignore)]
    #[bw(map = |x| x.parse::<u8>().unwrap())]
    pub art_desc: String
}

#[binrw]
#[derive(Serialize, Deserialize, Debug)]
pub struct GalleryArtParam {
    #[serde(skip)]
    pub version: u32,

    pub entry_count: u32,
  
    #[serde(skip)]
    pub entry_ptr: u64,

    #[br(count = entry_count)]
    pub entries: Vec<Entry>
}

impl NuccBinaryParsed for GalleryArtParam {
    fn binary_type(&self) -> NuccBinaryType {
        NuccBinaryType::GalleryArtParam
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

impl From<&[u8]> for GalleryArtParam {
    fn from(data: &[u8]) -> Self {
        let mut reader = Cursor::new(data);
        

        let version = reader.read_le::<u32>().unwrap();
        let entry_count = reader.read_le::<u32>().unwrap();
        let entry_ptr = reader.read_le::<u64>().unwrap();

        let mut entries = Vec::new();
        entries.reserve_exact(entry_count as usize); // Make sure we reserve enough space to avoid reallocations

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
            .map(|(i, e)| (((0x68 * i + HEADER_SIZE) as u64, e))) 
        {
            entry.art_id = read_string_from_ptr(&mut reader, entry.art_id_ptr, current_offset);
            entry.icon_path = read_string_from_ptr(&mut reader, entry.icon_path_ptr, current_offset + 0x8);
            entry.icon_filename = read_string_from_ptr(&mut reader, entry.icon_filename_ptr, current_offset + 0x10);
            entry.img_path = read_string_from_ptr(&mut reader, entry.img_path_ptr, current_offset + 0x18);
            entry.img_filename = read_string_from_ptr(&mut reader, entry.img_filename_ptr, current_offset + 0x20);
            entry.characode = read_string_from_ptr(&mut reader, entry.characode_ptr, current_offset + 0x30);
            entry.art_name = read_string_from_ptr(&mut reader, entry.art_name_ptr, current_offset + 0x50);
            entry.art_desc = read_string_from_ptr(&mut reader, entry.art_desc_ptr, current_offset + 0x58);
            
        }

        Self {
            version,
            entry_count,
            entry_ptr,
            entries
        }
    }
}

impl From<GalleryArtParam> for Vec<u8> {
    fn from(mut gallery_art_param: GalleryArtParam) -> Self {
        // Consumes the deserialized version and returns the bytes
        let mut writer = Cursor::new(Vec::new());

        gallery_art_param.entry_count = gallery_art_param.entries.len() as u32; // Update entry count

     
        writer.write_le(&1000u32).unwrap(); // Write the version
        writer.write_le(&gallery_art_param.entry_count).unwrap();
        writer.write_le(&8u64).unwrap(); // Write the ptr to the entries

        writer.write_le(&gallery_art_param.entries).unwrap();

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
        for (current_offset, entry) in gallery_art_param.entries
            .iter_mut()
            .enumerate()
            .map(|(i, e)| (((0x68 * i + HEADER_SIZE) as u64, e)))
        {
            write_ptr_to_string(&mut writer, &entry.art_id, current_offset, 0);
            write_ptr_to_string(&mut writer, &entry.icon_path, current_offset, 0x8);
            write_ptr_to_string(&mut writer, &entry.icon_filename, current_offset, 0x10);
            write_ptr_to_string(&mut writer, &entry.img_path, current_offset, 0x18);
            write_ptr_to_string(&mut writer, &entry.img_filename, current_offset, 0x20);
            write_ptr_to_string(&mut writer, &entry.characode, current_offset, 0x30);
            write_ptr_to_string(&mut writer, &entry.art_name, current_offset, 0x50);
            write_ptr_to_string(&mut writer, &entry.art_desc, current_offset, 0x58);
        }

 
        writer.into_inner()
    }
}



