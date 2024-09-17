mod args;
mod nucc_binary;

use std::{path::Path , fs::{self, create_dir_all}};
use std::collections::HashMap;


use strum::IntoEnumIterator;
use xfbin::{read_xfbin, write_xfbin};

use xfbin::nucc::*;
use xfbin::nucc_chunk::*;

use nucc_binary::*;
use nuccbin::NuccBinaryType;
use args::*;


pub fn main() {

    let args: Args = Args::new().unwrap();

    if let Ok(metadata) = fs::metadata(args.filepath.clone()) {

        if metadata.is_file() && args.extension == "xfbin" {
            // When the object is an xfbin file, unpack it
            unpack(args.clone());
        }

        if metadata.is_dir() {
            // When the object is a directory, repack it
            repack(args.clone());
        }
    }

}

fn find_nucc_binary_type(filepath: &String) -> Option<NuccBinaryType> {
    for nucc_binary_type in NuccBinaryType::iter() {
        let regex = nucc_binary_type.patterns();
        if regex.is_match(filepath) {
            return Some(nucc_binary_type);
        }
    }
    None
}

fn unpack(args: Args) {
    let xfbin = read_xfbin(&args.filepath).unwrap();

    // Create a directory with the name of the xfbin to store the serialized binary chunks
    let directory = Path::new(&args.filename);
    create_dir_all(directory).unwrap();
    
   
    for chunk in &xfbin.get_chunks_by_type(NuccChunkType::NuccChunkBinary) {
        let nucc_binary = chunk.downcast_ref::<NuccBinary>().unwrap();
        let data = nucc_binary.data.clone();

        if let Some(nucc_binary_type) = find_nucc_binary_type(&nucc_binary.struct_info.filepath) { 
            let reader = NuccBinaryParsedReader(nucc_binary_type, &data);
            let nucc_binary_parsed: Box<dyn NuccBinaryParsed> = reader.into();
            let ext = nucc_binary_parsed.extension();

            fs::write(directory.join(format!("{}{}", &nucc_binary.struct_info.chunk_name, ext)), nucc_binary_parsed.serialize()).unwrap();

        }
    }
}


fn repack(args: Args) {
    // Find the xfbin in the target folder with the same name as the folder
    let dir = Path::new(&args.directory);
    let xfbin_filepath = dir.join(format!("{}.xfbin", args.filepath.file_name().unwrap().to_str().unwrap()));

    let mut xfbin = read_xfbin(&xfbin_filepath).unwrap();

    let mut filepath_map = HashMap::new();

    for entry in fs::read_dir(&args.filepath).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let filepath = path.to_str().unwrap().to_string();
        
        // Extract the chunk name from the filename
        let chunk_name = path.file_stem().unwrap().to_str().unwrap().to_string();
        filepath_map.insert(chunk_name, filepath);
    }

    let mut chunks = xfbin.get_chunks_by_type(NuccChunkType::NuccChunkBinary);

    let mut nucc_binaries = Vec::new(); // The updated chunks that will replace the old chunks in the xfbin

    for chunk in &mut chunks {
        let nucc_binary = chunk.downcast_ref::<NuccBinary>().unwrap();
        let chunk_info = nucc_binary.struct_info.clone();
        let chunk_name = &chunk_info.chunk_name;

        if let Some(filepath) = filepath_map.get(chunk_name) {
            if let Some(nucc_binary_type) = find_nucc_binary_type(&chunk_info.filepath) {
                let serialized = fs::read(filepath).unwrap(); // Read each serialized binary chunk

                let deserializer = NuccBinaryParsedDeserializer(nucc_binary_type, serialized);
                let mut nucc_binary = nucc_binary.clone();
                
                nucc_binary.struct_info = chunk_info;
                nucc_binary.data = NuccBinaryParsedWriter(deserializer.into()).into();

                nucc_binaries.push(nucc_binary);
            }
        } else {
            println!("No matching file found for chunk: {}", chunk_name);
        }
    }

    // Update the xfbin with the new chunks based on the matching struct_info
    for n in nucc_binaries {
        for page in &mut xfbin.pages {
            for chunk in &mut page.structs {
                if n.struct_info == chunk.downcast_mut::<NuccBinary>().unwrap().struct_info  {
                    chunk.downcast_mut::<NuccBinary>().unwrap().data = n.data.clone();
                }
            }
        }
        
    }

    
    
    write_xfbin(xfbin, &xfbin_filepath.as_path()).unwrap();
}

