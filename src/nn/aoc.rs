#[allow(unused_imports)]
use self::super::root;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3aoc16ListAddOnContentEPiii"]
    pub fn ListAddOnContent(out_indices: *mut i32, offset: i32, count: i32);
}