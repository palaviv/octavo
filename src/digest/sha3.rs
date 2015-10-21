use digest;
use utils::buffer;

use byteorder::{
    ReadBytesExt,
    WriteBytesExt,
    LittleEndian,
};

use std::io::Read;

struct State {
    hash: [u64; 25],
    message: [u8; 144],
    rest: usize,
    block_size: usize,
}

const ROUND_CONSTS: [u64; 24] = [
    0x0000000000000001, 0x0000000000008082, 0x800000000000808a, 0x8000000080008000,
    0x000000000000808b, 0x0000000080000001, 0x8000000080008081, 0x8000000000008009,
    0x000000000000008a, 0x0000000000000088, 0x0000000080008009, 0x000000008000000a,
    0x000000008000808b, 0x800000000000008b, 0x8000000000008089, 0x8000000000008003,
    0x8000000000008002, 0x8000000000000080, 0x000000000000800a, 0x800000008000000a,
    0x8000000080008081, 0x8000000000008080, 0x0000000080000001, 0x8000000080008008
];

const SHIFTS: [u32; 25] = [
    0,   1,   62,  28,  27,
    36,  44,  6,   55,  20,
    3,   10,  43,  25,  39,
    41,  45,  15,  21,  8,
    18,  2,   61,  56,  14,
];

impl State {
    fn init(bits: usize) -> Self {
        let rate = 1600 - bits * 2;
        assert!(rate <= 1600 && (rate % 64) == 0);
        State {
            hash: [0; 25],
            message: [0; 144],
            rest: 0,
            block_size: rate / 8,
        }
    }

    #[allow(needless_range_loop)]
    fn theta(&mut self) {
        let mut a = [0u64; 5];
        let mut b = [0u64; 5];

        for i in 0..5 {
            a[i] = self.hash[i]
                ^ self.hash[i + 5]
                ^ self.hash[i + 10]
                ^ self.hash[i + 15]
                ^ self.hash[i + 20];
        }

        for i in 0..5 {
            b[i] = a[(i + 1) % 5].rotate_left(1) ^ a[(i + 4) % 5];
        }

        for i in 0..5 {
            self.hash[i]      ^= b[i];
            self.hash[i + 5]  ^= b[i];
            self.hash[i + 10] ^= b[i];
            self.hash[i + 15] ^= b[i];
            self.hash[i + 20] ^= b[i];
        }
    }

    fn rho(&mut self) {
        for (i, shift) in SHIFTS.iter().enumerate() {
            self.hash[i] = self.hash[i].rotate_left(*shift);
        }
    }

    fn pi(&mut self) {
        let tmp       = self.hash[ 1];
        self.hash[ 1] = self.hash[ 6];
        self.hash[ 6] = self.hash[ 9];
        self.hash[ 9] = self.hash[22];
        self.hash[22] = self.hash[14];
        self.hash[14] = self.hash[20];
        self.hash[20] = self.hash[ 2];
        self.hash[ 2] = self.hash[12];
        self.hash[12] = self.hash[13];
        self.hash[13] = self.hash[19];
        self.hash[19] = self.hash[23];
        self.hash[23] = self.hash[15];
        self.hash[15] = self.hash[ 4];
        self.hash[ 4] = self.hash[24];
        self.hash[24] = self.hash[21];
        self.hash[21] = self.hash[ 8];
        self.hash[ 8] = self.hash[16];
        self.hash[16] = self.hash[ 5];
        self.hash[ 5] = self.hash[ 3];
        self.hash[ 3] = self.hash[18];
        self.hash[18] = self.hash[17];
        self.hash[17] = self.hash[11];
        self.hash[11] = self.hash[ 7];
        self.hash[ 7] = self.hash[10];
        self.hash[10] = tmp;
        // NOTE: self.hash[0] is left untouched
    }

    fn chi(&mut self) {
        for i in 0..5 {
            let i = i * 5;
            let tmp_0 = self.hash[i];
            let tmp_1 = self.hash[i + 1];

            self.hash[i]     ^= !tmp_1 & self.hash[i + 2];
            self.hash[i + 1] ^= !self.hash[i + 2] & self.hash[i + 3];
            self.hash[i + 2] ^= !self.hash[i + 3] & self.hash[i + 4];
            self.hash[i + 3] ^= !self.hash[i + 4] & tmp_0;
            self.hash[i + 4] ^= !tmp_0 & tmp_1;
        }
    }

    fn permutation(&mut self) {
        for round in &ROUND_CONSTS {
            self.theta();
            self.rho();
            self.pi();
            self.chi();

            // iota
            self.hash[0] ^= *round;
        }
    }

    fn process(&mut self, mut data: &[u8]) {
        for i in 0..9 {
            self.hash[i] ^= data.read_u64::<LittleEndian>().unwrap();
        }
        if self.block_size > 72 {
            for i in 9..13 {
                self.hash[i] ^= data.read_u64::<LittleEndian>().unwrap();
            }
        }
        if self.block_size > 104 {
            for i in 13..17 {
                self.hash[i] ^= data.read_u64::<LittleEndian>().unwrap();
            }
        }
        if self.block_size > 138 {
            self.hash[17] ^= data.read_u64::<LittleEndian>().unwrap();
        }

        self.permutation();
    }

    fn update(&mut self, mut data: &[u8]) {
        while let Ok(len) = data.read(&mut self.message[self.rest..self.block_size]) {
            if len + self.rest < self.block_size {
                self.rest = len;
                return
            }
            assert!(len + self.rest == self.block_size);
            let message = self.message;
            self.process(&message[..]);
            self.rest = 0;
        }
    }

    fn finish(&mut self) {
        buffer::zero(&mut self.message[self.rest..self.block_size]);
        self.message[self.rest] |= 0x06;
        self.message[self.block_size - 1] |= 0x80;

        let message = self.message;
        self.process(&message[..]);
    }
}

macro_rules! sha3_impl {
    ($name:ident -> $size:expr) => {
        pub struct $name {
            state: State
        }

        impl Default for $name {
            fn default() -> Self {
                $name { state: State::init($size) }
            }
        }

        impl digest::Digest for $name {
            fn update<T>(&mut self, data: T) where T: AsRef<[u8]> {
                self.state.update(data.as_ref());
            }

            fn output_bits() -> usize { $size }
            fn block_size() -> usize { 1600 - (2 * $size) }

            fn result<T>(mut self, mut out: T) where T: AsMut<[u8]> {
                let mut ret = out.as_mut();
                assert!(ret.len() >= Self::output_bytes());

                self.state.finish();

                let mut tmp = [0u8; 200];
                {
                    let mut p = &mut tmp[..];

                    for i in 0..25 {
                        p.write_u64::<LittleEndian>(self.state.hash[i]).unwrap();
                    }
                }

                for i in 0..(Self::output_bytes()) {
                    ret[i] = tmp[i];
                }
            }
        }
    }
}

sha3_impl!(Sha3224 -> 224);
sha3_impl!(Sha3256 -> 256);
sha3_impl!(Sha3384 -> 384);
sha3_impl!(Sha3512 -> 512);

#[cfg(test)]
mod tests {
    mod sha3224 {
        use digest::sha3::Sha3224;
        use digest::test::Test;

        const TESTS: &'static [Test<'static>] = &[
            Test { input: b"", output: &[ 0x6b, 0x4e, 0x03, 0x42, 0x36, 0x67, 0xdb, 0xb7, 0x3b, 0x6e, 0x15, 0x45, 0x4f, 0x0e, 0xb1, 0xab, 0xd4, 0x59, 0x7f, 0x9a, 0x1b, 0x07, 0x8e, 0x3f, 0x5b, 0x5a, 0x6b, 0xc7,  ] },
            Test { input: b"a", output: &[ 0x9e, 0x86, 0xff, 0x69, 0x55, 0x7c, 0xa9, 0x5f, 0x40, 0x5f, 0x08, 0x12, 0x69, 0x68, 0x5b, 0x38, 0xe3, 0xa8, 0x19, 0xb3, 0x09, 0xee, 0x94, 0x2f, 0x48, 0x2b, 0x6a, 0x8b,  ] },
            Test { input: b"abc", output: &[ 0xe6, 0x42, 0x82, 0x4c, 0x3f, 0x8c, 0xf2, 0x4a, 0xd0, 0x92, 0x34, 0xee, 0x7d, 0x3c, 0x76, 0x6f, 0xc9, 0xa3, 0xa5, 0x16, 0x8d, 0x0c, 0x94, 0xad, 0x73, 0xb4, 0x6f, 0xdf,  ] },
            Test { input: b"message digest", output: &[ 0x18, 0x76, 0x8b, 0xb4, 0xc4, 0x8e, 0xb7, 0xfc, 0x88, 0xe5, 0xdd, 0xb1, 0x7e, 0xfc, 0xf2, 0x96, 0x4a, 0xbd, 0x77, 0x98, 0xa3, 0x9d, 0x86, 0xa4, 0xb4, 0xa1, 0xe4, 0xc8,  ] },
            Test { input: b"abcdefghijklmnopqrstuvwxyz", output: &[ 0x5c, 0xde, 0xca, 0x81, 0xe1, 0x23, 0xf8, 0x7c, 0xad, 0x96, 0xb9, 0xcb, 0xa9, 0x99, 0xf1, 0x6f, 0x6d, 0x41, 0x54, 0x96, 0x08, 0xd4, 0xe0, 0xf4, 0x68, 0x1b, 0x82, 0x39,  ] },
            Test { input: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", output: &[ 0xa6, 0x7c, 0x28, 0x9b, 0x82, 0x50, 0xa6, 0xf4, 0x37, 0xa2, 0x01, 0x37, 0x98, 0x5d, 0x60, 0x55, 0x89, 0xa8, 0xc1, 0x63, 0xd4, 0x52, 0x61, 0xb1, 0x54, 0x19, 0x55, 0x6e,  ] },
            Test { input: b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", output: &[ 0x05, 0x26, 0x89, 0x8e, 0x18, 0x58, 0x69, 0xf9, 0x1b, 0x3e, 0x2a, 0x76, 0xdd, 0x72, 0xa1, 0x5d, 0xc6, 0x94, 0x0a, 0x67, 0xc8, 0x16, 0x4a, 0x04, 0x4c, 0xd2, 0x5c, 0xc8,  ] },
            Test { input: b"The quick brown fox jumps over the lazy dog", output: &[ 0xd1, 0x5d, 0xad, 0xce, 0xaa, 0x4d, 0x5d, 0x7b, 0xb3, 0xb4, 0x8f, 0x44, 0x64, 0x21, 0xd5, 0x42, 0xe0, 0x8a, 0xd8, 0x88, 0x73, 0x05, 0xe2, 0x8d, 0x58, 0x33, 0x57, 0x95,  ] },
        ];

        #[test]
        fn simple_test_vectors() {
            for test in TESTS {
                test.test(Sha3224::default());
            }
        }
    }

    mod sha3256 {
        use digest::sha3::Sha3256;
        use digest::test::Test;

        const TESTS: &'static [Test<'static>] = &[
            Test { input: b"", output: &[ 0xa7, 0xff, 0xc6, 0xf8, 0xbf, 0x1e, 0xd7, 0x66, 0x51, 0xc1, 0x47, 0x56, 0xa0, 0x61, 0xd6, 0x62, 0xf5, 0x80, 0xff, 0x4d, 0xe4, 0x3b, 0x49, 0xfa, 0x82, 0xd8, 0x0a, 0x4b, 0x80, 0xf8, 0x43, 0x4a,  ] },
            Test { input: b"a", output: &[ 0x80, 0x08, 0x4b, 0xf2, 0xfb, 0xa0, 0x24, 0x75, 0x72, 0x6f, 0xeb, 0x2c, 0xab, 0x2d, 0x82, 0x15, 0xea, 0xb1, 0x4b, 0xc6, 0xbd, 0xd8, 0xbf, 0xb2, 0xc8, 0x15, 0x12, 0x57, 0x03, 0x2e, 0xcd, 0x8b,  ] },
            Test { input: b"abc", output: &[ 0x3a, 0x98, 0x5d, 0xa7, 0x4f, 0xe2, 0x25, 0xb2, 0x04, 0x5c, 0x17, 0x2d, 0x6b, 0xd3, 0x90, 0xbd, 0x85, 0x5f, 0x08, 0x6e, 0x3e, 0x9d, 0x52, 0x5b, 0x46, 0xbf, 0xe2, 0x45, 0x11, 0x43, 0x15, 0x32,  ] },
            Test { input: b"message digest", output: &[ 0xed, 0xcd, 0xb2, 0x06, 0x93, 0x66, 0xe7, 0x52, 0x43, 0x86, 0x0c, 0x18, 0xc3, 0xa1, 0x14, 0x65, 0xec, 0xa3, 0x4b, 0xce, 0x61, 0x43, 0xd3, 0x0c, 0x86, 0x65, 0xce, 0xfc, 0xfd, 0x32, 0xbf, 0xfd,  ] },
            Test { input: b"abcdefghijklmnopqrstuvwxyz", output: &[ 0x7c, 0xab, 0x2d, 0xc7, 0x65, 0xe2, 0x1b, 0x24, 0x1d, 0xbc, 0x1c, 0x25, 0x5c, 0xe6, 0x20, 0xb2, 0x9f, 0x52, 0x7c, 0x6d, 0x5e, 0x7f, 0x5f, 0x84, 0x3e, 0x56, 0x28, 0x8f, 0x0d, 0x70, 0x75, 0x21,  ] },
            Test { input: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", output: &[ 0xa7, 0x9d, 0x6a, 0x9d, 0xa4, 0x7f, 0x04, 0xa3, 0xb9, 0xa9, 0x32, 0x3e, 0xc9, 0x99, 0x1f, 0x21, 0x05, 0xd4, 0xc7, 0x8a, 0x7b, 0xc7, 0xbe, 0xeb, 0x10, 0x38, 0x55, 0xa7, 0xa1, 0x1d, 0xfb, 0x9f,  ] },
            Test { input: b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", output: &[ 0x29, 0x3e, 0x5c, 0xe4, 0xce, 0x54, 0xee, 0x71, 0x99, 0x0a, 0xb0, 0x6e, 0x51, 0x1b, 0x7c, 0xcd, 0x62, 0x72, 0x2b, 0x1b, 0xeb, 0x41, 0x4f, 0x5f, 0xf6, 0x5c, 0x82, 0x74, 0xe0, 0xf5, 0xbe, 0x1d,  ] },
            Test { input: b"The quick brown fox jumps over the lazy dog", output: &[ 0x69, 0x07, 0x0d, 0xda, 0x01, 0x97, 0x5c, 0x8c, 0x12, 0x0c, 0x3a, 0xad, 0xa1, 0xb2, 0x82, 0x39, 0x4e, 0x7f, 0x03, 0x2f, 0xa9, 0xcf, 0x32, 0xf4, 0xcb, 0x22, 0x59, 0xa0, 0x89, 0x7d, 0xfc, 0x04,  ] },
        ];

        #[test]
        fn simple_test_vectors() {
            for test in TESTS {
                test.test(Sha3256::default());
            }
        }
    }

    mod sha3384 {
        use digest::sha3::Sha3384;
        use digest::test::Test;

        const TESTS: &'static [Test<'static>] = &[
            Test { input: b"", output: &[ 0x0c, 0x63, 0xa7, 0x5b, 0x84, 0x5e, 0x4f, 0x7d, 0x01, 0x10, 0x7d, 0x85, 0x2e, 0x4c, 0x24, 0x85, 0xc5, 0x1a, 0x50, 0xaa, 0xaa, 0x94, 0xfc, 0x61, 0x99, 0x5e, 0x71, 0xbb, 0xee, 0x98, 0x3a, 0x2a, 0xc3, 0x71, 0x38, 0x31, 0x26, 0x4a, 0xdb, 0x47, 0xfb, 0x6b, 0xd1, 0xe0, 0x58, 0xd5, 0xf0, 0x04,  ] },
            Test { input: b"a", output: &[ 0x18, 0x15, 0xf7, 0x74, 0xf3, 0x20, 0x49, 0x1b, 0x48, 0x56, 0x9e, 0xfe, 0xc7, 0x94, 0xd2, 0x49, 0xee, 0xb5, 0x9a, 0xae, 0x46, 0xd2, 0x2b, 0xf7, 0x7d, 0xaf, 0xe2, 0x5c, 0x5e, 0xdc, 0x28, 0xd7, 0xea, 0x44, 0xf9, 0x3e, 0xe1, 0x23, 0x4a, 0xa8, 0x8f, 0x61, 0xc9, 0x19, 0x12, 0xa4, 0xcc, 0xd9,  ] },
            Test { input: b"abc", output: &[ 0xec, 0x01, 0x49, 0x82, 0x88, 0x51, 0x6f, 0xc9, 0x26, 0x45, 0x9f, 0x58, 0xe2, 0xc6, 0xad, 0x8d, 0xf9, 0xb4, 0x73, 0xcb, 0x0f, 0xc0, 0x8c, 0x25, 0x96, 0xda, 0x7c, 0xf0, 0xe4, 0x9b, 0xe4, 0xb2, 0x98, 0xd8, 0x8c, 0xea, 0x92, 0x7a, 0xc7, 0xf5, 0x39, 0xf1, 0xed, 0xf2, 0x28, 0x37, 0x6d, 0x25,  ] },
            Test { input: b"message digest", output: &[ 0xd9, 0x51, 0x97, 0x09, 0xf4, 0x4a, 0xf7, 0x3e, 0x2c, 0x8e, 0x29, 0x11, 0x09, 0xa9, 0x79, 0xde, 0x3d, 0x61, 0xdc, 0x02, 0xbf, 0x69, 0xde, 0xf7, 0xfb, 0xff, 0xdf, 0xff, 0xe6, 0x62, 0x75, 0x15, 0x13, 0xf1, 0x9a, 0xd5, 0x7e, 0x17, 0xd4, 0xb9, 0x3b, 0xa1, 0xe4, 0x84, 0xfc, 0x19, 0x80, 0xd5,  ] },
            Test { input: b"abcdefghijklmnopqrstuvwxyz", output: &[ 0xfe, 0xd3, 0x99, 0xd2, 0x21, 0x7a, 0xaf, 0x4c, 0x71, 0x7a, 0xd0, 0xc5, 0x10, 0x2c, 0x15, 0x58, 0x9e, 0x1c, 0x99, 0x0c, 0xc2, 0xb9, 0xa5, 0x02, 0x90, 0x56, 0xa7, 0xf7, 0x48, 0x58, 0x88, 0xd6, 0xab, 0x65, 0xdb, 0x23, 0x70, 0x07, 0x7a, 0x5c, 0xad, 0xb5, 0x3f, 0xc9, 0x28, 0x0d, 0x27, 0x8f,  ] },
            Test { input: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", output: &[ 0xd5, 0xb9, 0x72, 0x30, 0x2f, 0x50, 0x80, 0xd0, 0x83, 0x0e, 0x0d, 0xe7, 0xb6, 0xb2, 0xcf, 0x38, 0x36, 0x65, 0xa0, 0x08, 0xf4, 0xc4, 0xf3, 0x86, 0xa6, 0x11, 0x12, 0x65, 0x2c, 0x74, 0x2d, 0x20, 0xcb, 0x45, 0xaa, 0x51, 0xbd, 0x4f, 0x54, 0x2f, 0xc7, 0x33, 0xe2, 0x71, 0x9e, 0x99, 0x92, 0x91,  ] },
            Test { input: b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", output: &[ 0x3c, 0x21, 0x3a, 0x17, 0xf5, 0x14, 0x63, 0x8a, 0xcb, 0x3b, 0xf1, 0x7f, 0x10, 0x9f, 0x3e, 0x24, 0xc1, 0x6f, 0x9f, 0x14, 0xf0, 0x85, 0xb5, 0x2a, 0x2f, 0x2b, 0x81, 0xad, 0xc0, 0xdb, 0x83, 0xdf, 0x1a, 0x58, 0xdb, 0x2c, 0xe0, 0x13, 0x19, 0x1b, 0x8b, 0xa7, 0x2d, 0x8f, 0xae, 0x7e, 0x2a, 0x5e,  ] },
            Test { input: b"The quick brown fox jumps over the lazy dog", output: &[ 0x70, 0x63, 0x46, 0x5e, 0x08, 0xa9, 0x3b, 0xce, 0x31, 0xcd, 0x89, 0xd2, 0xe3, 0xca, 0x8f, 0x60, 0x24, 0x98, 0x69, 0x6e, 0x25, 0x35, 0x92, 0xed, 0x26, 0xf0, 0x7b, 0xf7, 0xe7, 0x03, 0xcf, 0x32, 0x85, 0x81, 0xe1, 0x47, 0x1a, 0x7b, 0xa7, 0xab, 0x11, 0x9b, 0x1a, 0x9e, 0xbd, 0xf8, 0xbe, 0x41,  ] },
        ];

        #[test]
        fn simple_test_vectors() {
            for test in TESTS {
                test.test(Sha3384::default());
            }
        }
    }

    mod sha3512 {
        use digest::sha3::Sha3512;
        use digest::test::Test;

        const TESTS: &'static [Test<'static>] = &[
            Test { input: b"", output: &[ 0xa6, 0x9f, 0x73, 0xcc, 0xa2, 0x3a, 0x9a, 0xc5, 0xc8, 0xb5, 0x67, 0xdc, 0x18, 0x5a, 0x75, 0x6e, 0x97, 0xc9, 0x82, 0x16, 0x4f, 0xe2, 0x58, 0x59, 0xe0, 0xd1, 0xdc, 0xc1, 0x47, 0x5c, 0x80, 0xa6, 0x15, 0xb2, 0x12, 0x3a, 0xf1, 0xf5, 0xf9, 0x4c, 0x11, 0xe3, 0xe9, 0x40, 0x2c, 0x3a, 0xc5, 0x58, 0xf5, 0x00, 0x19, 0x9d, 0x95, 0xb6, 0xd3, 0xe3, 0x01, 0x75, 0x85, 0x86, 0x28, 0x1d, 0xcd, 0x26,  ] },
            Test { input: b"a", output: &[ 0x69, 0x7f, 0x2d, 0x85, 0x61, 0x72, 0xcb, 0x83, 0x09, 0xd6, 0xb8, 0xb9, 0x7d, 0xac, 0x4d, 0xe3, 0x44, 0xb5, 0x49, 0xd4, 0xde, 0xe6, 0x1e, 0xdf, 0xb4, 0x96, 0x2d, 0x86, 0x98, 0xb7, 0xfa, 0x80, 0x3f, 0x4f, 0x93, 0xff, 0x24, 0x39, 0x35, 0x86, 0xe2, 0x8b, 0x5b, 0x95, 0x7a, 0xc3, 0xd1, 0xd3, 0x69, 0x42, 0x0c, 0xe5, 0x33, 0x32, 0x71, 0x2f, 0x99, 0x7b, 0xd3, 0x36, 0xd0, 0x9a, 0xb0, 0x2a,  ] },
            Test { input: b"abc", output: &[ 0xb7, 0x51, 0x85, 0x0b, 0x1a, 0x57, 0x16, 0x8a, 0x56, 0x93, 0xcd, 0x92, 0x4b, 0x6b, 0x09, 0x6e, 0x08, 0xf6, 0x21, 0x82, 0x74, 0x44, 0xf7, 0x0d, 0x88, 0x4f, 0x5d, 0x02, 0x40, 0xd2, 0x71, 0x2e, 0x10, 0xe1, 0x16, 0xe9, 0x19, 0x2a, 0xf3, 0xc9, 0x1a, 0x7e, 0xc5, 0x76, 0x47, 0xe3, 0x93, 0x40, 0x57, 0x34, 0x0b, 0x4c, 0xf4, 0x08, 0xd5, 0xa5, 0x65, 0x92, 0xf8, 0x27, 0x4e, 0xec, 0x53, 0xf0,  ] },
            Test { input: b"message digest", output: &[ 0x34, 0x44, 0xe1, 0x55, 0x88, 0x1f, 0xa1, 0x55, 0x11, 0xf5, 0x77, 0x26, 0xc7, 0xd7, 0xcf, 0xe8, 0x03, 0x02, 0xa7, 0x43, 0x30, 0x67, 0xb2, 0x9d, 0x59, 0xa7, 0x14, 0x15, 0xca, 0x9d, 0xd1, 0x41, 0xac, 0x89, 0x2d, 0x31, 0x0b, 0xc4, 0xd7, 0x81, 0x28, 0xc9, 0x8f, 0xda, 0x83, 0x9d, 0x18, 0xd7, 0xf0, 0x55, 0x6f, 0x2f, 0xe7, 0xac, 0xb3, 0xc0, 0xcd, 0xa4, 0xbf, 0xf3, 0xa2, 0x5f, 0x5f, 0x59,  ] },
            Test { input: b"abcdefghijklmnopqrstuvwxyz", output: &[ 0xaf, 0x32, 0x8d, 0x17, 0xfa, 0x28, 0x75, 0x3a, 0x3c, 0x9f, 0x5c, 0xb7, 0x2e, 0x37, 0x6b, 0x90, 0x44, 0x0b, 0x96, 0xf0, 0x28, 0x9e, 0x57, 0x03, 0xb7, 0x29, 0x32, 0x4a, 0x97, 0x5a, 0xb3, 0x84, 0xed, 0xa5, 0x65, 0xfc, 0x92, 0xaa, 0xde, 0xd1, 0x43, 0x66, 0x99, 0x00, 0xd7, 0x61, 0x86, 0x16, 0x87, 0xac, 0xdc, 0x0a, 0x5f, 0xfa, 0x35, 0x8b, 0xd0, 0x57, 0x1a, 0xaa, 0xd8, 0x0a, 0xca, 0x68,  ] },
            Test { input: b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789", output: &[ 0xd1, 0xdb, 0x17, 0xb4, 0x74, 0x5b, 0x25, 0x5e, 0x5e, 0xb1, 0x59, 0xf6, 0x65, 0x93, 0xcc, 0x9c, 0x14, 0x38, 0x50, 0x97, 0x9f, 0xc7, 0xa3, 0x95, 0x17, 0x96, 0xab, 0xa8, 0x01, 0x65, 0xaa, 0xb5, 0x36, 0xb4, 0x61, 0x74, 0xce, 0x19, 0xe3, 0xf7, 0x07, 0xf0, 0xe5, 0xc6, 0x48, 0x7f, 0x5f, 0x03, 0x08, 0x4b, 0xc0, 0xec, 0x94, 0x61, 0x69, 0x1e, 0xf2, 0x01, 0x13, 0xe4, 0x2a, 0xd2, 0x81, 0x63,  ] },
            Test { input: b"12345678901234567890123456789012345678901234567890123456789012345678901234567890", output: &[ 0x95, 0x24, 0xb9, 0xa5, 0x53, 0x6b, 0x91, 0x06, 0x95, 0x26, 0xb4, 0xf6, 0x19, 0x6b, 0x7e, 0x94, 0x75, 0xb4, 0xda, 0x69, 0xe0, 0x1f, 0x0c, 0x85, 0x57, 0x97, 0xf2, 0x24, 0xcd, 0x73, 0x35, 0xdd, 0xb2, 0x86, 0xfd, 0x99, 0xb9, 0xb3, 0x2f, 0xfe, 0x33, 0xb5, 0x9a, 0xd4, 0x24, 0xcc, 0x17, 0x44, 0xf6, 0xeb, 0x59, 0x13, 0x7f, 0x5f, 0xb8, 0x60, 0x19, 0x32, 0xe8, 0xa8, 0xaf, 0x0a, 0xe9, 0x30,  ] },
            Test { input: b"The quick brown fox jumps over the lazy dog", output: &[ 0x01, 0xde, 0xdd, 0x5d, 0xe4, 0xef, 0x14, 0x64, 0x24, 0x45, 0xba, 0x5f, 0x5b, 0x97, 0xc1, 0x5e, 0x47, 0xb9, 0xad, 0x93, 0x13, 0x26, 0xe4, 0xb0, 0x72, 0x7c, 0xd9, 0x4c, 0xef, 0xc4, 0x4f, 0xff, 0x23, 0xf0, 0x7b, 0xf5, 0x43, 0x13, 0x99, 0x39, 0xb4, 0x91, 0x28, 0xca, 0xf4, 0x36, 0xdc, 0x1b, 0xde, 0xe5, 0x4f, 0xcb, 0x24, 0x02, 0x3a, 0x08, 0xd9, 0x40, 0x3f, 0x9b, 0x4b, 0xf0, 0xd4, 0x50,  ] },
        ];


        #[test]
        fn simple_test_vectors() {
            for test in TESTS {
                test.test(Sha3512::default());
            }
        }
    }
}
