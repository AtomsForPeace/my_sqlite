use std::array;

use crate::constant::{PAGE_SIZE, TABLE_MAX_PAGES, TABLE_MAX_ROWS};

pub struct Table {
    pub num_rows: usize,
    pub pages: [Option<Box<[u8; PAGE_SIZE]>>; TABLE_MAX_PAGES],
}

impl Table {
    pub fn new() -> Self {
        Self {
            num_rows: TABLE_MAX_ROWS,
            pages: array::from_fn(|_| None),
        }
    }

    pub fn allocate_page(&mut self, page_num: usize) {
        if page_num < self.pages.len() {
            self.pages[page_num] = Some(Box::new([0u8; PAGE_SIZE])); // Allocate memory for the page
        } else {
            println!("Page number out of bounds");
        }
    }
}
