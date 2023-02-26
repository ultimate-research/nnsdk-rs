extern "C" {
    #[link_name = "\u{1}_ZN2nn7account6detail13IsInitializedEv"]
    pub fn IsInitialized() -> bool;
}

pub fn is_initialized() -> bool {
    let result = unsafe { IsInitialized() };
    return result;
}