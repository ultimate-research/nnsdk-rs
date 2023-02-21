#[allow(unused_imports)]
use alloc::{vec, vec::Vec};

extern "C" {
    #[link_name = "\u{1}_ZN2nn3aoc17CountAddOnContentEv"]
    pub fn CountAddOnContent() -> usize;

    #[link_name = "\u{1}_ZN2nn3aoc16ListAddOnContentEPiii"]
    pub fn ListAddOnContent(out_indices: *mut i32, offset: i32, count: usize);
}

pub fn count_add_on_content() -> usize{
    unsafe {
        return CountAddOnContent();
    }
}

pub fn list_add_on_content(offset: i32) -> Vec<i32> {
    let count = count_add_on_content();
    let mut out = vec![0; count];
    unsafe {
        ListAddOnContent(out.as_mut_ptr(), offset, count)
    }
    return out
}