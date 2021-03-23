use std::{fs, path::PathBuf};

use colored::Colorize;
use tar::{Builder, Header};
use walkdir::WalkDir;

use crate::{compressors::Compressor, error::{Error, OuchResult}, file::File};

use super::compressor::Entry;

pub struct TarCompressor {}

impl TarCompressor {

    fn make_archive_from_memory(input: File) -> OuchResult<Vec<u8>> {
        
        let contents = match input.contents {
            Some(bytes) => bytes,
            None => {
                eprintln!("{}: reached TarCompressor::make_archive_from_memory without known content.", "internal error".red());
                return Err(Error::InvalidInput);
            }
        };

        let mut header = Header::new_gnu();
        
        header.set_path(&input.path).unwrap();
        header.set_size(contents.len() as u64);
        header.set_cksum();


        let mut b = Builder::new(Vec::new());
        b.append_data(&mut header, &input.path, &*contents)?;

        Ok(b.into_inner()?)
    }

    fn make_archive_from_files(input_filenames: Vec<PathBuf>) -> OuchResult<Vec<u8>> {
    
        let buf = Vec::new();
        let mut b = Builder::new(buf);
    
        for filename in input_filenames {
            for entry in WalkDir::new(&filename) {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    continue;
                }
                b.append_file(path, &mut fs::File::open(path)?)?;
            }
        }
        
        Ok(b.into_inner()?)
    }
}

impl Compressor for TarCompressor {
    fn compress(&self, from: Entry) -> OuchResult<Vec<u8>> {

        match from {
            Entry::Files(filenames) => {
                Ok(
                    Self::make_archive_from_files(filenames)?
                )
            },
            Entry::InMemory(file) => {
                Ok(
                    Self::make_archive_from_memory(file)?
                )
            }
        }        
    }
}