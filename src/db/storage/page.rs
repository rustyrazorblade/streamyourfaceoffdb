use std::io::Cursor;
use std::io::{Read, Write};

use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};
use db::row::Row;

// mis info about what's on each page.
pub const PAGE_SIZE : usize = 4096;

pub struct Page {
//    data: [u8; PAGE_SIZE - HEADER_SIZE_IN_BYTES],
    data: Vec<u8>,
    bytes_used: usize, // tracking where we are in the current page

                                      // if we go past the page boundary on a write we fault
}

#[derive(Debug)]
pub enum PageError {
    Full,
    NotFound,
}

pub type PageResult<T> = Result<T, PageError>;

// page deals with bytes, and has zero knowledge of a Row
// storage engine needs to serialize
impl Page {
    pub fn new() -> Page {
        let tmp: [u8; PAGE_SIZE];

        Page{data: Vec::with_capacity(PAGE_SIZE),
             bytes_used: 0,
             }
    }

    // attempt to write to a page
    // can fail if the page is already full
    // if it fails, the storage engine will have to flush this page
    // then allocate a new page
    pub fn write(&mut self, bytes: &[u8]) -> PageResult<()> {
        if bytes.len() > self.space_available() {
            return Err(PageError::Full)
        }
        // we need the length of the row
        let len = bytes.len() as u16;
        self.data.write_u16::<BigEndian>(len);
        self.data.write(bytes);

        self.bytes_used += 2 + len as usize;

        // TODO: update indexes
        Ok(())
    }

    // internal call, try to write instead since it holds a mutable ref
    fn space_available(&self) -> usize {
        PAGE_SIZE - self.bytes_used
    }

    // returns PAGE_SIZE bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(PAGE_SIZE);
        // get the header
        result.extend(&self.data);
        result.resize(PAGE_SIZE, 0);
        result
    }
    pub fn from_bytes(bytes: Vec<u8>) -> PageResult<Page> {
        let p = Page{bytes_used: 0,
                     data: bytes};
        Ok(p)
    }

}



#[cfg(test)]
mod tests {
    use super::{Page, PAGE_SIZE};
    #[test]
    fn test_page_insert_ok() {
        let mut p = Page::new();

        assert_eq!(p.space_available(), PAGE_SIZE);
        assert_eq!(p.bytes_used, 0);

        let data: [u8; 16] = [0; 16];
        p.write(&data).expect("Data written");
        // 2 extra bytes from the size
        assert_eq!(p.bytes_used, 18);

        let result = p.to_bytes();
        assert_eq!(result.len(), PAGE_SIZE);
    }

    #[test]
    #[should_panic]
    fn test_page_fault() {
        let mut p = Page::new();
        let data: [u8; 5000] = [0; 5000];
        p.write(&data).expect("Failure is expected");
    }
}

