#[doc(hidden)]
pub fn marshal<F>(m: &[u8],
                  padding: &[u8],
                  f: F
                  ) -> Option<Vec<u8>>
where F: Fn(&mut [u8]) -> Option<&[u8]> {
    let mut buf = Vec::with_capacity(padding.len() + m.len());
    buf.push_all(padding);
    buf.push_all(m);
    let c = match f(&mut buf) {
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

        ));

macro_rules! newtype_drop (($newtype:ident) => (
        impl Drop for $newtype {
            fn drop(&mut self) {
                let &mut $newtype(ref mut v) = self;
                unsafe {
                    volatile_set_memory(v.as_mut_ptr(), 0, v.len());
                }
            }
        }
        ));

macro_rules! newtype_impl (($newtype:ident, $len:expr) => (
    impl $newtype {
        /**
         * `from_slice()` creates an object from a byte slice
         *
         * This function will fail and return None if the length of
         * the byte-slice isn't equal to the length of the object
         */
        pub fn from_slice(bs: &[u8]) -> Option<$newtype> {
            if bs.len() != $len {
                return None
            }
            let mut n = $newtype([0; $len]);
            {
                let $newtype(ref mut b) = n;
                for (bi, &bsi) in b.iter_mut().zip(bs.iter()) {
                    *bi = bsi
                }
            }
            Some(n)
        }
    }
    /**
     * Allows a user to access the byte contents of an object as a slice.
     *
     * WARNING: it might be tempting to do comparisons on objects
     * by using `x[a..b] == y[a..b]`. This will open up for timing attacks
     * when comparing for example authenticator tags. Because of this only
     * use the comparison functions exposed by the sodiumoxide API.
     */
    impl Index<Range<usize>> for $newtype {
        type Output = [u8];
        fn index(&self, _index: Range<usize>) -> &[u8] {
            let &$newtype(ref b) = self;
            b.index(_index)
        }
    }
    /**
     * Allows a user to access the byte contents of an object as a slice.
     *
     * WARNING: it might be tempting to do comparisons on objects
     * by using `x[..b] == y[..b]`. This will open up for timing attacks
     * when comparing for example authenticator tags. Because of this only
     * use the comparison functions exposed by the sodiumoxide API.
     */
    impl Index<RangeTo<usize>> for $newtype {
        type Output = [u8];
        fn index(&self, _index: RangeTo<usize>) -> &[u8] {
            let &$newtype(ref b) = self;
            b.index(_index)
        }
    }
    /**
     * Allows a user to access the byte contents of an object as a slice.
     *
     * WARNING: it might be tempting to do comparisons on objects
     * by using `x[a..] == y[a..]`. This will open up for timing attacks
     * when comparing for example authenticator tags. Because of this only
     * use the comparison functions exposed by the sodiumoxide API.
     */
    impl Index<RangeFrom<usize>> for $newtype {
        type Output = [u8];
        fn index(&self, _index: RangeFrom<usize>) -> &[u8] {
            let &$newtype(ref b) = self;
            b.index(_index)
        }
    }
    /**
     * Allows a user to access the byte contents of an object as a slice.
     *
     * WARNING: it might be tempting to do comparisons on objects
     * by using `x[] == y[]`. This will open up for timing attacks
     * when comparing for example authenticator tags. Because of this only
     * use the comparison functions exposed by the sodiumoxide API.
     */
    impl Index<RangeFull> for $newtype {
        type Output = [u8];
        fn index(&self, _index: RangeFull) -> &[u8] {
            let &$newtype(ref b) = self;
            b.index(_index)
        }
    }
    ));
