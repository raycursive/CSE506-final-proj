use std::marker::PhantomData;

use crate::fix_sized_key::AsRaw;

#[derive(Debug, Clone, Copy)]
pub struct QuickIStr<'a> {
    buf: [u8; 32],
    len: usize,
    _phantom: PhantomData<&'a str>,
}

impl<'a> QuickIStr<'a> {
    pub fn new(v: u64) -> Self {
        let mut buf: [u8; 32] = [0; 32];
        let mut v_ = v;
        let mut i = 32 - 1;
        while v_ > 0 {
            buf[i] = ((v_ % 10) + 48) as u8;
            v_ /= 10;
            i -= 1;
        }
        let mut buf_ = [0; 32];
        buf_[0..32 - i - 1].copy_from_slice(buf[i + 1..].as_ref());
        QuickIStr {
            buf: buf_,
            len: 32 - i - 1,
            _phantom: PhantomData,
        }
    }
}

impl<'a> ToString for QuickIStr<'a> {
    #[inline(always)]
    fn to_string(&self) -> String {
        unsafe {
            return String::from_utf8_unchecked(self.buf[0..self.len].to_vec());
        }
    }
}

impl<'a> From<QuickIStr<'a>> for String {
    #[inline(always)]
    fn from(v: QuickIStr<'a>) -> Self {
        return v.to_string();
    }
}

impl<'a> AsRaw for QuickIStr<'a> {
    #[inline(always)]
    fn as_raw(&self) -> &[u8] {
        return self.buf[0..self.len].as_ref();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn quick_istr_works() {
        let v = QuickIStr::new(10024);
        println!("{:?}", v.to_string());
        println!("{:?}", Into::<String>::into(v));
    }
}
