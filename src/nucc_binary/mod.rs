mod anm_offset;
mod characode;
mod command_list_param;
mod custom_card_param;
mod customize_default_param;
mod dds;
mod dictionary_param;
mod dlc_info_param;
mod effectprm;
mod ev;
mod fcv;
mod gallery_art_param;
mod gion;
mod lip;
mod lua;
mod message_info;
mod player_color_param;
mod png;
mod prm_load;
mod sound_test_param;
mod speaking_line_param;
mod support_chara_param;
mod xml;


use binrw::{BinReaderExt, BinWriterExt};
use binrw::io::Cursor;
use downcast_rs::{impl_downcast, Downcast};
use super::NuccBinaryType;

pub const HEADER_SIZE: usize = 0x10; // Size of NUCC Binary headers

pub use anm_offset::Anmofs;
pub use characode::Characode;
pub use customize_default_param::CustomizeDefaultParam;
pub use command_list_param::CommandListParam;
pub use custom_card_param::CustomCardParam;
pub use dds::Dds;
pub use dictionary_param::DictionaryParam;
pub use dlc_info_param::DlcInfoParam;
pub use ev::Ev;
pub use fcv::Fcv;
pub use effectprm::EffectPrm;
pub use gallery_art_param::GalleryArtParam;
pub use gion::Gion;
pub use lip::Lip;
pub use lua::Lua;
pub use message_info::MessageInfo;
pub use player_color_param::PlayerColorParam;
pub use png::Png;
pub use prm_load::PrmLoad;
pub use sound_test_param::SoundTestParam;
pub use speaking_line_param::SpeakingLineParam;
pub use support_chara_param::SupportCharaParam;
pub use xml::Xml;


pub trait NuccBinaryParsed: Downcast {
    fn binary_type(&self) -> NuccBinaryType;
    fn extension(&self) -> String;
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self
    where 
        Self: Sized;
}

impl_downcast!(NuccBinaryParsed);


pub struct NuccBinaryParsedReader<'a> (pub NuccBinaryType, pub &'a [u8]);

impl From<NuccBinaryParsedReader<'_>> for Box<dyn NuccBinaryParsed> {
    fn from(reader: NuccBinaryParsedReader<'_>) -> Self {
        let NuccBinaryParsedReader(nucc_binary_type, data) = reader;

        match nucc_binary_type {
            NuccBinaryType::Anmofs => {
                let mut anmofs = Cursor::new(data);
                Box::new(anmofs.read_le::<Anmofs>().unwrap())
            },

            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(data);
                Box::new(characode.read_le::<Characode>().unwrap())
            },

            NuccBinaryType::CommandListParam => Box::new(CommandListParam::from(&data[..])),
            NuccBinaryType::CustomCardParam => Box::new(CustomCardParam::from(&data[..])),
            NuccBinaryType::CustomizeDefaultParam => Box::new(CustomizeDefaultParam::from(&data[..])),
            NuccBinaryType::Dds => Box::new(Dds::from(&data[..])),
            NuccBinaryType::DictionaryParam => Box::new(DictionaryParam::from(&data[..])),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::from(&data[..])),
            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(data);
                Box::new(effect_prm.read_le::<EffectPrm>().unwrap())
            },

            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(data);
                Box::new(ev.read_le::<Ev>().unwrap())
            },

            NuccBinaryType::Fcv => Box::new(Fcv::from(&data[..])),

            NuccBinaryType::GalleryArtParam => Box::new(GalleryArtParam::from(&data[..])),
            NuccBinaryType::Gion => {
                let mut gion = Cursor::new(data);
                Box::new(gion.read_le::<Gion>().unwrap())
            },

            NuccBinaryType::Lip => {
                let mut lip = Cursor::new(data);
                Box::new(lip.read_le::<Lip>().unwrap())
            },

            NuccBinaryType::Lua => Box::new(Lua::from(&data[..])),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::from(&data[..])),
            NuccBinaryType::PlayerColorParam => Box::new(PlayerColorParam::from(&data[..])),

            NuccBinaryType::Png => Box::new(Png::from(&data[..])),
            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(data);
                Box::new(prm_load.read_le::<PrmLoad>().unwrap())
            }, 

            NuccBinaryType::SoundTestParam => Box::new(SoundTestParam::from(&data[..])),
            NuccBinaryType::SpeakingLineParam => Box::new(SpeakingLineParam::from(&data[..])),
            
         

            NuccBinaryType::SupportCharaParam => Box::new(SupportCharaParam::from(&data[..])),

            

            NuccBinaryType::Xml => Box::new(Xml::from(&data[..])),
        }
    }
}

pub struct NuccBinaryParsedWriter(pub Box<dyn NuccBinaryParsed>);

impl From<NuccBinaryParsedWriter> for Vec<u8> {
    fn from(writer: NuccBinaryParsedWriter) -> Self {
        let NuccBinaryParsedWriter(boxed) = writer;
        
        match boxed.binary_type() {
            NuccBinaryType::Anmofs => {
                let mut anmofs = Cursor::new(Vec::new());
                anmofs.write_le(&*boxed.downcast::<Anmofs>().ok().unwrap()).unwrap();
                anmofs.into_inner()
            },
                
            NuccBinaryType::Characode => {
                let mut characode = Cursor::new(Vec::new());
                characode.write_le(&*boxed.downcast::<Characode>().ok().unwrap()).unwrap();
                characode.into_inner()
            },

            NuccBinaryType::CommandListParam => { (*boxed.downcast::<CommandListParam>().ok().unwrap()).into() },
            NuccBinaryType::CustomCardParam => { (*boxed.downcast::<CustomCardParam>().ok().unwrap()).into() },
            NuccBinaryType::CustomizeDefaultParam => { (*boxed.downcast::<CustomizeDefaultParam>().ok().unwrap()).into() },
            NuccBinaryType::Dds => { (*boxed.downcast::<Dds>().ok().unwrap()).into() },
            NuccBinaryType::DictionaryParam => { (*boxed.downcast::<DictionaryParam>().ok().unwrap()).into() },
            NuccBinaryType::DlcInfoParam => { (*boxed.downcast::<DlcInfoParam>().ok().unwrap()).into() },

            NuccBinaryType::EffectPrm => {
                let mut effect_prm = Cursor::new(Vec::new());
                effect_prm.write_le(&*boxed.downcast::<EffectPrm>().ok().unwrap()).unwrap();
                effect_prm.into_inner()
            },
            NuccBinaryType::Ev => {
                let mut ev = Cursor::new(Vec::new());
                ev.write_le(&*boxed.downcast::<Ev>().ok().unwrap()).unwrap();
                ev.into_inner()
            },

            NuccBinaryType::Fcv => { (*boxed.downcast::<Fcv>().ok().unwrap()).into() },

            NuccBinaryType::GalleryArtParam => { (*boxed.downcast::<GalleryArtParam>().ok().unwrap()).into() },

            NuccBinaryType::Gion => {
                let mut gion = Cursor::new(Vec::new());
                gion.write_le(&*boxed.downcast::<Characode>().ok().unwrap()).unwrap();
                gion.into_inner()
            },

            NuccBinaryType::Lip => {
                let mut lip = Cursor::new(Vec::new());
                lip.write_le(&*boxed.downcast::<Lip>().ok().unwrap()).unwrap();
                lip.into_inner()
            },

            NuccBinaryType::Lua => { (*boxed.downcast::<Lua>().ok().unwrap()).into() },
            NuccBinaryType::MessageInfo => { (*boxed.downcast::<MessageInfo>().ok().unwrap()).into() },
            NuccBinaryType::PlayerColorParam => { (*boxed.downcast::<PlayerColorParam>().ok().unwrap()).into() },
            NuccBinaryType::Png => { (*boxed.downcast::<Png>().ok().unwrap()).into() },

            NuccBinaryType::PrmLoad => {
                let mut prm_load = Cursor::new(Vec::new());
                prm_load.write_le(&*boxed.downcast::<PrmLoad>().ok().unwrap()).unwrap();
                prm_load.into_inner()
            },

            NuccBinaryType::SoundTestParam => { (*boxed.downcast::<SoundTestParam>().ok().unwrap()).into() },
            NuccBinaryType::SpeakingLineParam => { (*boxed.downcast::<SpeakingLineParam>().ok().unwrap()).into() },
            NuccBinaryType::SupportCharaParam => { (*boxed.downcast::<SupportCharaParam>().ok().unwrap()).into() },
            NuccBinaryType::Xml => { (*boxed.downcast::<Xml>().ok().unwrap()).into() }
        }
    }
}

pub struct NuccBinaryParsedSerializer(pub Box<dyn NuccBinaryParsed>);

impl From<NuccBinaryParsedSerializer> for Vec<u8> {
    fn from(serializer: NuccBinaryParsedSerializer) -> Self {
        let NuccBinaryParsedSerializer(nucc_binary_parsed) = serializer;
        nucc_binary_parsed.serialize()
    }
}

#[derive(Debug)]
pub struct NuccBinaryParsedDeserializer(pub NuccBinaryType, pub Vec<u8>);

impl From<NuccBinaryParsedDeserializer> for Box<dyn NuccBinaryParsed> {
    fn from(deserializer: NuccBinaryParsedDeserializer) -> Self {
       let NuccBinaryParsedDeserializer(nucc_binary_type, data) = deserializer;

        match nucc_binary_type {
            NuccBinaryType::Anmofs => Box::new(Anmofs::deserialize(&data)),
            NuccBinaryType::Characode => Box::new(Characode::deserialize(&data)),
            NuccBinaryType::CommandListParam => Box::new(CommandListParam::deserialize(&data)),
            NuccBinaryType::CustomCardParam => Box::new(CustomCardParam::deserialize(&data)),
            NuccBinaryType::CustomizeDefaultParam => Box::new(CustomizeDefaultParam::deserialize(&data)),
            NuccBinaryType::Dds => Box::new(Dds::deserialize(&data)),
            NuccBinaryType::DictionaryParam => Box::new(DictionaryParam::deserialize(&data)),
            NuccBinaryType::DlcInfoParam => Box::new(DlcInfoParam::deserialize(&data)),
            NuccBinaryType::EffectPrm => Box::new(EffectPrm::deserialize(&data)),
            NuccBinaryType::Ev => Box::new(Ev::deserialize(&data)),
            NuccBinaryType::Fcv => Box::new(Fcv::deserialize(&data)),
            NuccBinaryType::GalleryArtParam => Box::new(GalleryArtParam::deserialize(&data)),
            NuccBinaryType::Gion => Box::new(Gion::deserialize(&data)),
            NuccBinaryType::Lip => Box::new(Lip::deserialize(&data)),
            NuccBinaryType::Lua => Box::new(Lua::deserialize(&data)),
            NuccBinaryType::MessageInfo => Box::new(MessageInfo::deserialize(&data)),
            NuccBinaryType::PlayerColorParam => Box::new(PlayerColorParam::deserialize(&data)),
            NuccBinaryType::Png => Box::new(Png::deserialize(&data)),
            NuccBinaryType::PrmLoad => Box::new(PrmLoad::deserialize(&data)),
            NuccBinaryType::SoundTestParam => Box::new(SoundTestParam::deserialize(&data)),
            NuccBinaryType::SpeakingLineParam => Box::new(SpeakingLineParam::deserialize(&data)),
            NuccBinaryType::SupportCharaParam => Box::new(SupportCharaParam::deserialize(&data)),
            NuccBinaryType::Xml => Box::new(Xml::deserialize(&data))
        }
    }
}





