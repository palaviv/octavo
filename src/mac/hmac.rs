use std::io::Write;

use digest::Digest;
use mac::Mac;

pub struct Hmac<T: Digest + Default> {
    digest: T,
    key: Vec<u8>,
}

impl<T: Digest + Default> Hmac<T> {
    pub fn new<K: AsRef<[u8]>>(key: K) -> Self {
        Self::with_digest(key.as_ref(), Default::default())
    }

    pub fn with_digest(key: &[u8], mut digest: T) -> Self {
        let key = Self::expand_key(key);
        let ikey: Vec<_> = key.iter().map(|&b| b ^ 0x36).collect();

        digest.update(&ikey);

        Hmac {
            digest: digest,
            key: key,
        }
    }

    fn expand_key(key: &[u8]) -> Vec<u8> {
        let bs = T::block_size();
        let mut exp_key = vec![0; bs];

        if key.len() <= bs {
            exp_key.write(key).unwrap();
        } else {
            let mut digest = T::default();
            digest.update(key);
            digest.result(&mut exp_key[..bs]);
        }

        exp_key
    }

}

impl<T: Digest + Default> Mac for Hmac<T> {
    fn update<D: AsRef<[u8]>>(&mut self, data: D) {
        self.digest.update(data)
    }

    fn output_bits() -> usize { T::output_bits() }
    fn block_size() -> usize { T::block_size() }

    fn result<O: AsMut<[u8]>>(mut self, mut output: O) {
        self.digest.result(output.as_mut());
        self.digest = T::default();

        let okey: Vec<_> = self.key.iter().map(|&b| b ^ 0x5c).collect();

        self.digest.update(okey);
        self.digest.update(output.as_mut());
        self.digest.result(output.as_mut());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mac::Mac;
    use digest::Digest;
    use digest::md5::Md5;

    #[test]
    fn test_empty_string() {
        let mut hmac_md5 = Hmac::<Md5>::new("");
        hmac_md5.update("");

        let mut output = [0; 16];

        hmac_md5.result(&mut output);

        assert_eq!([0x74, 0xe6, 0xf7, 0x29, 0x8a, 0x9c, 0x2d, 0x16, 0x89, 0x35, 0xf5, 0x8c, 0x00, 0x1b, 0xad, 0x88], output);
    }
}
