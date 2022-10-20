use std::{
    io::{self, Cursor, Read, Seek, Write},
    path::Path,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use tar::Header;

use crate::{course_decryptor::decrypt_course_data, level_parser::Level};

pub struct Archive<R: Read> {
    archive: tar::Archive<GzDecoder<R>>,
}

impl<R: Read> Archive<R> {
    pub fn new(reader: R) -> Self {
        Self {
            archive: tar::Archive::new(GzDecoder::new(reader)),
        }
    }

    pub fn read(&mut self) -> io::Result<ArchiveReader<R>> {
        Ok(ArchiveReader {
            entries: self.archive.entries()?,
        })
    }
}

pub struct ArchiveReader<'a, R: Read> {
    entries: tar::Entries<'a, GzDecoder<R>>,
}

impl<'a, R: Read> ArchiveReader<'a, R> {
    pub(crate) fn next_raw(&mut self) -> Option<io::Result<tar::Entry<'a, GzDecoder<R>>>> {
        self.entries.next()
    }

    pub fn next_level(&mut self) -> Option<io::Result<Level>>
    where
        R: Seek,
    {
        self.next_raw().map(|option| {
            option.and_then(|mut entry| {
                let mut data: Vec<u8> = Vec::with_capacity(entry.size() as usize);
                entry.read_to_end(&mut data)?;
                Level::parse(&mut Cursor::new(data))
            })
        })
    }
}

pub struct ArchiveWriter<W: Write> {
    tar: tar::Builder<flate2::write::GzEncoder<W>>,
    header: Header,
}

impl<W: Write> ArchiveWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            tar: tar::Builder::new(GzEncoder::new(writer, Compression::fast())),
            header: Header::new_gnu(),
        }
    }

    pub fn append_archive<R: Read>(&mut self, mut archive: Archive<R>) -> io::Result<()> {
        let mut archive = archive.read()?;

        while let Some(entry_result) = archive.next_raw() {
            let mut entry = entry_result?;
            // unfortunately, entry cannot be split into a header borrow and a reader borrow.
            let header = entry.header().clone();
            self.tar.append(&header, &mut entry)?;
        }
        Ok(())
    }

    pub fn append_decrypted_level<P>(&mut self, path: P, data: &[u8]) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        self.header.set_size(data.len().try_into().unwrap());
        self.header.set_cksum();
        self.tar.append_data(&mut self.header, path, data)
    }

    pub fn append_encrypted_level<P>(&mut self, level_code: &str, data: &[u8]) -> io::Result<()>
    where
        P: AsRef<Path>,
    {
        let decrypted_data = decrypt_course_data(data);
        self.append_decrypted_level(level_code, decrypted_data.as_slice())
    }

    pub fn finish(self) -> io::Result<()> {
        self.tar.into_inner()?;
        Ok(())
    }
}
