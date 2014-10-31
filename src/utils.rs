#![macro_escape]

#[doc(hidden)]
pub fn marshal<'a>(m: &'a [u8],
                   padding: &[u8],
                   f: |&mut [u8]| -> Option<&'a [u8]>
                   ) -> Option<Vec<u8>> {
    let mut buf = Vec::with_capacity(padding.len() + m.len());
    buf.push_all(padding);
    buf.push_all(m);
    let c = match f(buf.as_mut_slice()) {
        None => return None,
        Some(c) => c
    };
    let mut dst = Vec::with_capacity(c.len());
    dst.push_all(c);
    Some(dst)
}

macro_rules! newtype_clone (($newtype:ident) => (

        impl Clone for $newtype {
            fn clone(&self) -> $newtype {
                let &$newtype(v) = self;
                $newtype(v)
            }
        }

        ))

macro_rules! newtype_drop (($newtype:ident) => (
        impl Drop for $newtype {
            fn drop(&mut self) {
                let &$newtype(ref mut v) = self;
                unsafe {
                    volatile_set_memory(v.as_mut_ptr(), 0, v.len());
                }
            }
        }
        ))
