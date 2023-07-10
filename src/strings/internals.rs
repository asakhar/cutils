pub use core;

pub const fn panic_on_invalid_utf8(slice: &[u8]) {
  if core::str::from_utf8(slice).is_err() {
    panic!("Invalid utf-8");
  }
}

pub const fn check_is_valid_utf8(slice: &[u8]) -> Result<(), core::str::Utf8Error> {
  match core::str::from_utf8(slice) {
    Err(err) => Err(err),
    _ => Ok(()),
  }
}

pub fn encode_u8(utf8: &str) -> Option<Vec<u8>> {
  utf8.chars().map(| c| {
    if c as u32 > (u8::MAX as u32) {
      None
    } else {
      Some(c as u8)
    }
  }).collect()
}

pub fn encode_u16(utf8: &str) -> Option<Vec<u16>> {
  utf8.chars().map(|c| {
    if c as u32 > (u16::MAX as u32) {
      None
    } else {
      Some(c as u16)
    }
  }).collect()
}

pub fn encode_u32(utf8: &str) -> Vec<u32> {
  utf8.chars().map(|c| {
    c as u32
  }).collect()
}

pub fn decode_u8(data: &[u8]) -> Option<String> {
  data.iter().copied().map(Into::into).map(char::from_u32).collect()
}

pub fn decode_u16(data: &[u16]) -> Option<String> {
  data.iter().copied().map(Into::into).map(char::from_u32).collect()
}

pub fn decode_u32(data: &[u32]) -> Option<String> {
  data.iter().copied().map(char::from_u32).collect()
}

// A const implementation of https://github.com/rust-lang/rust/blob/d902752866cbbdb331e3cf28ff6bba86ab0f6c62/library/core/src/str/mod.rs#L509-L537
// Assumes `utf8` is a valid &str
pub const unsafe fn next_code_point(utf8: &[u8]) -> Option<(u32, &[u8])> {
  const CONT_MASK: u8 = 0b0011_1111;
  match utf8 {
    [one @ 0..=0b0111_1111, rest @ ..] => Some((*one as u32, rest)),
    [one @ 0b1100_0000..=0b1101_1111, two, rest @ ..] => Some((
      (((*one & 0b0001_1111) as u32) << 6) | ((*two & CONT_MASK) as u32),
      rest,
    )),
    [one @ 0b1110_0000..=0b1110_1111, two, three, rest @ ..] => Some((
      (((*one & 0b0000_1111) as u32) << 12)
        | (((*two & CONT_MASK) as u32) << 6)
        | ((*three & CONT_MASK) as u32),
      rest,
    )),
    [one, two, three, four, rest @ ..] => Some((
      (((*one & 0b0000_0111) as u32) << 18)
        | (((*two & CONT_MASK) as u32) << 12)
        | (((*three & CONT_MASK) as u32) << 6)
        | ((*four & CONT_MASK) as u32),
      rest,
    )),
    [..] => None,
  }
}

// A const implementation of `s.chars().map(|ch| ch.len_utf16()).sum()`
pub const unsafe fn length_as_utf16(mut bytes: &[u8]) -> usize {
  let mut len = 0;
  while let Some((ch, rest)) = unsafe { next_code_point(bytes) } {
    bytes = rest;
    len += if (ch & 0xFFFF) == ch { 1 } else { 2 };
  }
  len
}

pub const fn length_as_utf8(bytes: &[u8]) -> usize {
  bytes.len()
}

pub const unsafe fn length_as_u16(mut bytes: &[u8]) -> Result<usize, usize> {
  let mut len = 0;
  while let Some((ch, rest)) = next_code_point(bytes) {
    bytes = rest;
    if (ch & 0xFFFF) != ch {
      return Err(len);
    }
    len += 1;
  }
  Ok(len)
}

pub const unsafe fn length_as_u16_or_panic(s: &[u8]) -> usize {
  match length_as_u16(s) {
    Ok(res) => res,
    _ => panic!("Invalid input for u16 string"),
  }
}

pub const unsafe fn length_as_u8(mut bytes: &[u8]) -> Result<usize, usize> {
  let mut len = 0;
  while let Some((ch, rest)) = next_code_point(bytes) {
    bytes = rest;
    if (ch & 0xFF) != ch {
      return Err(len);
    }
    len += 1;
  }
  Ok(len)
}

pub const unsafe fn length_as_u8_or_panic(s: &[u8]) -> usize {
  match length_as_u8(s) {
    Ok(res) => res,
    _ => panic!("Invalid input for u8 string"),
  }
}

// A const implementation of `s.chars().len()`
pub const unsafe fn length_as_u32(mut bytes: &[u8]) -> Result<usize, usize> {
  let mut len = 0;
  while let Some((_, rest)) = next_code_point(bytes) {
    bytes = rest;
    len += 1;
  }
  Ok(len)
}

pub const unsafe fn length_as_u32_or_panic(mut bytes: &[u8]) -> usize {
  let mut len = 0;
  while let Some((_, rest)) = next_code_point(bytes) {
    bytes = rest;
    len += 1;
  }
  len
}
