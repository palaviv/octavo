#![feature(test)]

extern crate octavo_digest as digest;
extern crate test;

#[macro_export]
macro_rules! bench_digest {
    ($name:ident, $engine:path, $bs:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use digest::Digest;

            let mut d = <$engine>::default();
            let data = vec![0; $bs];

            b.iter(|| {
                d.update(&data);
            });

            b.bytes = $bs;
        }
    };

    ($engine:path) => {
        bench_digest!(_16_block_size,   $engine,   16);
        bench_digest!(_64_block_size,   $engine,   64);
        bench_digest!(_256_block_size,  $engine,  256);
        bench_digest!(_1024_block_size, $engine, 1024);
        bench_digest!(_8192_block_size, $engine, 8192);
    }
}

#[cfg(feature = "md5")] #[macro_use]mod md5;
#[cfg(feature = "sha1")] #[macro_use]mod sha1;
#[cfg(feature = "sha2")] #[macro_use]mod sha2;
#[cfg(feature = "sha3")] #[macro_use]mod sha3;
#[cfg(feature = "tiger")] #[macro_use]mod tiger;
#[cfg(feature = "ripemd")] #[macro_use]mod ripemd;
#[cfg(feature = "blake2")] #[macro_use]mod blake2;
#[cfg(feature = "whirlpool")] #[macro_use]mod whirlpool;
