#![allow(warnings)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[inline]
pub fn LOWORD(l: DWORD) -> WORD {
    (l & 0xffff) as WORD
}

#[inline]
pub fn HIWORD(l: DWORD) -> WORD {
    ((l >> 16) & 0xffff) as WORD
}
