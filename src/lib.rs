use strum_macros::{EnumString, EnumIter, Display};
use regex::Regex;

#[derive(Debug, Copy, Clone, EnumString, EnumIter, Display, PartialEq)]
pub enum NuccBinaryType {
    Anmofs,
    #[strum(ascii_case_insensitive)]
    Characode,
    CommandListParam,
    CustomCardParam,
    CustomizeDefaultParam,
    Dds,
    DictionaryParam,
    DlcInfoParam,
    EffectPrm,
    Ev,
    Fcv,
    GalleryArtParam,
    Gion,
    Lip,
    Lua,
    MessageInfo,
    PlayerColorParam,
    Png,
    PrmLoad,
    SoundTestParam,
    SpeakingLineParam,
    SupportCharaParam,
    Xml,
}

impl NuccBinaryType {
    pub fn patterns(&self) -> Regex {

        match self {
            NuccBinaryType::Anmofs => { Regex::new(r"(anm_offset)").unwrap() },
            NuccBinaryType::Characode => { Regex::new(r"(characode\.bin)$").unwrap() },
            NuccBinaryType::CommandListParam => { Regex::new(r"(CommandListParam\.bin)$").unwrap() },
            NuccBinaryType::CustomCardParam => { Regex::new(r"(CustomCardParam\.bin)$").unwrap() },
            NuccBinaryType::CustomizeDefaultParam => { Regex::new(r"(CustomizeDefaultParam\.bin)$").unwrap() },
            NuccBinaryType::Dds => { Regex::new(r"(\.dds)$").unwrap() },
            NuccBinaryType::DictionaryParam => { Regex::new(r"(DictionaryParam\.bin)$").unwrap() },
            NuccBinaryType::DlcInfoParam => { Regex::new(r"(DlcInfoParam\.bin)$").unwrap() },
            NuccBinaryType::EffectPrm => { Regex::new(r"(effectprm.*\.bin)$").unwrap() },
            NuccBinaryType::Ev => { Regex::new(r"(_ev.bin)").unwrap() },
            NuccBinaryType::Fcv => { Regex::new(r"(\.fcv)$").unwrap() },
            NuccBinaryType::GalleryArtParam => { Regex::new(r"(GalleryArtParam\.bin)$").unwrap() },
            NuccBinaryType::Gion => { Regex::new(r"(gion.*\.bin)$").unwrap() },
            NuccBinaryType::Lip => { Regex::new(r"(_lip)").unwrap() },
            NuccBinaryType::Lua => { Regex::new(r"(\.lua)$").unwrap() },
            NuccBinaryType::MessageInfo => { Regex::new(r"(message)").unwrap() },
            NuccBinaryType::PlayerColorParam => { Regex::new(r"(PlayerColorParam\.bin)$").unwrap() },
            NuccBinaryType::Png => { Regex::new(r"(\.png)$").unwrap() },
            NuccBinaryType::PrmLoad => { Regex::new(r"(prm_load\.bin)$").unwrap() },
            NuccBinaryType::SoundTestParam => { Regex::new(r"(SoundTestParam\.bin)$").unwrap() },
            NuccBinaryType::SpeakingLineParam => { Regex::new(r"(SpeakingLineParam\.bin)$").unwrap() },
            NuccBinaryType::SupportCharaParam => { Regex::new(r"(SupportCharaParam\.bin)$").unwrap() },
            NuccBinaryType::Xml => { Regex::new(r"(\.xml)$").unwrap() }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::path::Path;
    use xfbin::read_xfbin;
    use super::NuccBinaryType;
    
    /*use super::nucc_binary::NuccBinaryParsed;

      
    #[test]
    fn characode_test() {
        let filepath = Path::new("9ind_x.xfbin");
        let xfbin = read_xfbin(filepath).unwrap();

        let _ = dbg!(xfbin.pages.len());

        for chunk in &xfbin.get_chunks_by_type("nuccChunkBinary") {
            let bytes = chunk.data.as_bytes();
            let reader = NuccBinaryParsedReader(NuccBinaryType::Characode, &bytes);
            let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = reader.into();
            let characode: &Characode = nucc_binary_parsed.as_any().downcast_ref::<Characode>().unwrap();
            characode.serialize(); // Serialize to JSON
        }
    }*/
}