#[cfg(windows)]
pub type WChar = u16;
#[cfg(not(windows))]
pub type WChar = u32;