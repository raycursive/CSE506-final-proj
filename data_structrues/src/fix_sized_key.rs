use std::marker::PhantomData;
use std::{
    fmt::{Debug, Display},
    str::Utf8Error,
};

pub trait AsRaw {
    fn as_raw(&self) -> &[u8];
}

impl AsRaw for String {
    #[inline(always)]
    fn as_raw(&self) -> &[u8] {
        return self.as_bytes();
    }
}

impl<'a> AsRaw for &'a str {
    #[inline(always)]
    fn as_raw(&self) -> &[u8] {
        return self.as_bytes();
    }
}

pub trait FixSizedKeyParams {
    const KEY_SIZE: usize;
    const ALLOW_INT_CMP: bool;
}

#[repr(C)]
union FixSizedKeyInner<T: FixSizedKeyParams>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    i_: [u64; T::KEY_SIZE / 8],
    c_: [u8; T::KEY_SIZE],
}

pub struct FixSizedKey<T: FixSizedKeyParams>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    key_u_: FixSizedKeyInner<T>,
    _phantom: PhantomData<T>,
}

impl<T: FixSizedKeyParams> Debug for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            f.write_str(
                format!(
                    "Key<{}>[\"{}\"; {}]",
                    T::KEY_SIZE,
                    &self.to_str().unwrap(),
                    &self
                        .key_u_
                        .i_
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(":")
                )
                .as_str(),
            )
        }
    }
}

impl<T: FixSizedKeyParams> Display for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.to_str().unwrap())
    }
}

impl<T: FixSizedKeyParams> PartialOrd for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if T::ALLOW_INT_CMP {
            unsafe {
                for i in 0..T::KEY_SIZE / 8 {
                    if self.key_u_.i_[i] < other.key_u_.i_[i] {
                        return Some(std::cmp::Ordering::Less);
                    } else if self.key_u_.i_[i] > other.key_u_.i_[i] {
                        return Some(std::cmp::Ordering::Greater);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        } else {
            unsafe {
                for i in 0..T::KEY_SIZE {
                    if self.key_u_.c_[i] < other.key_u_.c_[i] {
                        return Some(std::cmp::Ordering::Less);
                    } else if self.key_u_.c_[i] > other.key_u_.c_[i] {
                        return Some(std::cmp::Ordering::Greater);
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

impl<T: FixSizedKeyParams> PartialEq for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        return self.partial_cmp(other).unwrap() == std::cmp::Ordering::Equal;
    }
}

impl<T: FixSizedKeyParams> Eq for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
}

impl<T: FixSizedKeyParams> Ord for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.partial_cmp(other).unwrap();
    }
}

impl<T: FixSizedKeyParams> Clone for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    fn clone(&self) -> Self {
        let mut bytes_buf: [u64; T::KEY_SIZE / 8] = [0; T::KEY_SIZE / 8];
        unsafe {
            let dst_ptr = &mut bytes_buf as *mut u64;
            let src_ptr = &self.key_u_.i_ as *const u64;
            std::ptr::copy_nonoverlapping(src_ptr, dst_ptr, T::KEY_SIZE / 8);
        }
        Self {
            key_u_: FixSizedKeyInner { i_: bytes_buf },
            _phantom: PhantomData,
        }
    }
}
impl<T: FixSizedKeyParams> From<&[u8]> for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    fn from(value: &[u8]) -> Self {
        let mut buf: [u8; T::KEY_SIZE] = [0; T::KEY_SIZE];
        assert!(value.len() <= T::KEY_SIZE, "key size is too large");
        buf[..value.len()].copy_from_slice(value);
        return Self {
            key_u_: FixSizedKeyInner { c_: buf },
            _phantom: PhantomData,
        };
    }
}

impl<T: FixSizedKeyParams, K: AsRaw> From<K> for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    #[inline(always)]
    fn from(value: K) -> Self {
        return Self::from(value.as_raw());
    }
}

impl<T: FixSizedKeyParams> Into<String> for FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    #[inline(always)]
    fn into(self) -> String {
        return self.to_str().unwrap().to_string();
    }
}

impl<T: FixSizedKeyParams> FixSizedKey<T>
where
    [(); T::KEY_SIZE]: Sized,
    [(); T::KEY_SIZE / 8]: Sized,
{
    pub fn to_str(&self) -> Result<&str, Utf8Error> {
        unsafe {
            let mut cnt: usize = 0;
            for i in 0..T::KEY_SIZE {
                if self.key_u_.c_[i] == 0 {
                    break;
                }
                cnt += 1;
            }
            return std::str::from_utf8(&self.key_u_.c_[..cnt]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Params {}
    impl FixSizedKeyParams for Params {
        const KEY_SIZE: usize = 16;
        const ALLOW_INT_CMP: bool = true;
    }
    #[test]
    fn fix_sized_key_works() {
        type FixSizedKey_ = FixSizedKey<Params>;
        let k1: FixSizedKey_ = FixSizedKey_::from("hello");
        let k2 = "world".into();
        let k3 = k1.clone();
        assert_eq!(k1, k3);
        assert_ne!(k1, k2);
        assert!(k1 > k2);
        println!("{}", k1);
        println!("{}", k2);
    }
}
