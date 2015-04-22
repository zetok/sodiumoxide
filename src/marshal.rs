#[doc(hidden)]
pub fn marshal<F>(m: &[u8],
                  padding: &[u8],
                  f: F
                  ) -> Option<Vec<u8>>
where F: Fn(&mut [u8]) -> Option<&[u8]> {
    let mut buf = Vec::with_capacity(padding.len() + m.len());
    buf.extend(padding.iter().cloned());
    buf.extend(m.iter().cloned());
    let c = match f(&mut buf) {
        None => return None,
        Some(c) => c
    };
    let mut dst = Vec::with_capacity(c.len());
    dst.extend(c.iter().cloned());
    Some(dst)
}
