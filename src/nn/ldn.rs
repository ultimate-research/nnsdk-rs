#[allow(unused_imports)]
use self::super::root;

extern "C" {
    #[link_name = "\u{1}_ZN2nn3ldn10InitializeEv"]
    pub fn Initialize() -> root::Result;
}
extern "C" {
    #[link_name = "\u{1}_ZN2nn3ldn8FinalizeEv"]
    pub fn Finalize() -> root::Result;
}