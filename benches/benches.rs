use core::hash::Hasher;
use test::{black_box, Bencher};

use hashers::builtin::*;
use hashers::fnv::*;
use hashers::jenkins::spooky_hash::*;
use hashers::jenkins::*;
use hashers::null::*;
use hashers::oz::*;
use hashers::pigeon::*;

macro_rules! tiny_bench {
    ($name:ident, $fcn:ident, $hasher:ident) => {
        // hasher_to_fcn!($fcn, $hasher);
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| black_box($fcn(b"abcd")))
        }
    };
}

#[cfg(feature = "builtin")]
tiny_bench!(tiny_default, default, const_hashers::builtin::DefaultHasher);
#[cfg(feature = "oz")]
{
tiny_bench!(tiny_sdbm, sdbm, const_hashers::oz::SDBMHasher);
tiny_bench!(tiny_loselose, loselose, const_hashers::oz::LoseLoseHasher);
tiny_bench!(tiny_bricolage, const_hashers::oz::bricolage, const_hashers::oz::Bricolage);
tiny_bench!(tiny_djb2, djb2, const_hashers::oz::DJB2Hasher);
}
#[cfg(feature = "fnv")]
tiny_bench!(tiny_fnv1a64, fnv1a64, const_hashers::FNV1aHasher64);
#[cfg(feature = "jenkins")]
{
tiny_bench!(tiny_lookup3, lookup3, const_hashers::jenkins::Lookup3Hasher);
tiny_bench!(tiny_spooky, spooky, const_hashers::jenkins::SpookyHasher);
tiny_bench!(tiny_oaat, oaat, const_hashers::jenkins::OAATHasher);
}
#[cfg(feature = "null")]
tiny_bench!(tiny_passthrough, passthrough, const_hashers::PassThroughHasher);

macro_rules! w32_bench {
    ($name:ident, $hasher:ident, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut h = $hasher::default();
                for i in 0..$count {
                    h.write_i32(i);
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
w32_bench!(w32_10_default, const_hashers::builtin::DefaultHasher, 10);
#[cfg(feature = "oz")]
{
w32_bench!(w32_10_djb2, const_hashers::oz::DJB2Hasher, 10);
w32_bench!(w32_10_sdbm, const_hashers::oz::SDBMHasherHasher, 10);
w32_bench!(w32_10_bricolage, const_hashers::oz::Bricolage, 10);
w32_bench!(w32_10_loselose, const_hashers::oz::LoseLoseHasher, 10);
}
#[cfg(feature = "jenkins")]
{
w32_bench!(w32_10_oaat, const_hashers::jenkins::OAATHasher, 10);
w32_bench!(w32_10_lookup3, const_hashers::jenkins::Lookup3Hasher, 10);
w32_bench!(w32_10_spooky, const_hashers::jenkins::SpookyHasher, 10);
}
#[cfg(feature = "null")]
w32_bench!(w32_10_passthrough, const_hashers::null::PassThroughHasher, 10);
#[cfg(feature = "fnv")]
w32_bench!(w32_10_fnv1a64, const_hashers::fnv::FNV1aHasher64, 10);

#[cfg(feature = "builtin")]
w32_bench!(w32_100_default, const_hashers::builtin::DefaultHasher, 100);
#[cfg(feature = "oz")]
{
w32_bench!(w32_100_djb2, const_hashers::oz::DJB2Hasher, 100);
w32_bench!(w32_100_sdbm, const_hashers::oz::SDBMHasherHasher, 100);
w32_bench!(w32_100_loselose, const_hashers::oz::LoseLoseHasher, 100);
w32_bench!(w32_100_bricolage, const_hashers::oz::Bricolage, 100);
}
#[cfg(feature = "jenkins")]
{
w32_bench!(w32_100_spooky, const_hashers::jenkins::SpookyHasher, 100);
w32_bench!(w32_100_oaat, const_hashers::jenkins::OAATHasher, 100);
w32_bench!(w32_100_lookup3, const_hashers::jenkins::Lookup3Hasher, 100);
}
#[cfg(feature = "null")]
w32_bench!(w32_100_passthrough, const_hashers::null::PassThroughHasher, 100);
#[cfg(feature = "fnv")]
w32_bench!(w32_100_fnv1a64, const_hashers::fnv::FNV1aHasher64, 100);

#[cfg(feature = "builtin")]
w32_bench!(w32_1000_default, const_hashers::builtin::DefaultHasher, 1000);
#[cfg(feature = "oz")]
{
w32_bench!(w32_1000_bricolage, const_hashers::oz::Bricolage, 1000);
w32_bench!(w32_1000_djb2, const_hashers::oz::DJB2Hasher, 1000);
w32_bench!(w32_1000_sdbm, const_hashers::oz::SDBMHasherHasher, 1000);
w32_bench!(w32_1000_loselose, const_hashers::oz::LoseLoseHasher, 1000);
}
#[cfg(feature = "jenkins")]
{
w32_bench!(w32_1000_spooky, const_hashers::jenkins::SpookyHasher, 1000);

w32_bench!(w32_1000_oaat, const_hashers::jenkins::OAATHasher, 1000);
w32_bench!(w32_1000_lookup3, const_hashers::jenkins::Lookup3Hasher, 1000);
}
#[cfg(feature = "null")]
w32_bench!(w32_1000_passthrough, const_hashers::null::PassThroughHasher, 1000);
#[cfg(feature = "fnv")]
w32_bench!(w32_1000_fnv1a64, const_hashers::fnv::FNV1aHasher64, 1000);

macro_rules! w64_bench {
    ($name:ident, $hasher:ident, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut h = $hasher::default();
                for i in 0..$count {
                    h.write_i64(i);
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
w64_bench!(w64_10_default, const_hashers::builtin::DefaultHasher, 10);
#[cfg(feature = "oz")]
{
w64_bench!(w64_10_djb2, const_hashers::oz::DJB2Hasher, 10);
w64_bench!(w64_10_sdbm, const_hashers::oz::SDBMHasherHasher, 10);
w64_bench!(w64_10_loselose, const_hashers::oz::LoseLoseHasher, 10);
w64_bench!(w64_10_bricolage, const_hashers::oz::Bricolage, 10);
}
#[cfg(feature = "jenkins")]
{
w64_bench!(w64_10_oaat, const_hashers::jenkins::OAATHasher, 10);
w64_bench!(w64_10_lookup3, const_hashers::jenkins::Lookup3Hasher, 10);
w64_bench!(w64_10_spooky, const_hashers::jenkins::SpookyHasher, 10);
}
#[cfg(feature = "null")]
w64_bench!(w64_10_passthrough, const_hashers::null::PassThroughHasher, 10);
#[cfg(feature = "fnv")]
w64_bench!(w64_10_fnv1a64, const_hashers::fnv::FNV1aHasher64, 10);

#[cfg(feature = "builtin")]
w64_bench!(w64_100_default, const_hashers::builtin::DefaultHasher, 100);
#[cfg(feature = "oz")]
{
w64_bench!(w64_100_djb2, const_hashers::oz::DJB2Hasher, 100);
w64_bench!(w64_100_sdbm, const_hashers::oz::SDBMHasherHasher, 100);
w64_bench!(w64_100_loselose, const_hashers::oz::LoseLoseHasher, 100);
w64_bench!(w64_100_bricolage, const_hashers::oz::Bricolage, 100);
}
#[cfg(feature = "jenkins")]
{
w64_bench!(w64_100_oaat, const_hashers::jenkins::OAATHasher, 100);
w64_bench!(w64_100_lookup3, const_hashers::jenkins::Lookup3Hasher, 100);
w64_bench!(w64_100_spooky, const_hashers::jenkins::SpookyHasher, 100);
}
#[cfg(feature = "null")]
w64_bench!(w64_100_passthrough, const_hashers::null::PassThroughHasher, 100);
#[cfg(feature = "fnv")]
w64_bench!(w64_100_fnv1a64, const_hashers::fnv::FNV1aHasher64, 100);

#[cfg(feature = "builtin")]
w64_bench!(w64_1000_default, const_hashers::builtin::DefaultHasher, 1000);
#[cfg(feature = "oz")]
{
w64_bench!(w64_1000_djb2, const_hashers::oz::DJB2Hasher, 1000);
w64_bench!(w64_1000_sdbm, const_hashers::oz::SDBMHasherHasher, 1000);
w64_bench!(w64_1000_loselose, const_hashers::oz::LoseLoseHasher, 1000);
w64_bench!(w64_1000_bricolage, const_hashers::oz::Bricolage, 1000);
}
#[cfg(feature = "jenkins")]
{
w64_bench!(w64_1000_oaat, const_hashers::jenkins::OAATHasher, 1000);
w64_bench!(w64_1000_lookup3, const_hashers::jenkins::Lookup3Hasher, 1000);
w64_bench!(w64_1000_spooky, const_hashers::jenkins::SpookyHasher, 1000);
}
#[cfg(feature = "null")]
w64_bench!(w64_1000_passthrough, const_hashers::null::PassThroughHasher, 1000);
#[cfg(feature = "fnv")]
w64_bench!(w64_1000_fnv1a64, const_hashers::fnv::FNV1aHasher64, 1000);

fn read_words() -> Vec<String> {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    let file = File::open("./data/words.txt").expect("cannot open words.txt");
    return BufReader::new(file)
        .lines()
        .map(|l| l.expect("bad read"))
        .collect();
}

macro_rules! words_bench {
    ($name:ident, $hasher:ident, $count:expr) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let words = read_words();
            b.iter(|| {
                let mut h = $hasher::default();
                for i in words.iter().take($count) {
                    h.write(i.as_bytes());
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
words_bench!(words1000_default, const_hashers::builtin::DefaultHasher, 1000);
#[cfg(feature = "oz")]
{
words_bench!(words1000_djb2, const_hashers::oz::DJB2Hasher, 1000);
words_bench!(words1000_sdbm, const_hashers::oz::SDBMHasherHasher, 1000);
words_bench!(words1000_loselose, const_hashers::oz::LoseLoseHasher, 1000);
}
#[cfg(feature = "jenkins")]
{
words_bench!(words1000_spooky, const_hashers::jenkins::SpookyHasher, 1000);
words_bench!(words1000_oaat, const_hashers::jenkins::OAATHasher, 1000);
words_bench!(words1000_lookup3, const_hashers::jenkins::Lookup3Hasher, 1000);
}
#[cfg(feature = "null")]
words_bench!(words1000_passthrough, const_hashers::null::PassThroughHasher, 1000);
#[cfg(feature = "fnv")]
words_bench!(words1000_fnv1a64, const_hashers::fnv::FNV1aHasher64, 1000);

macro_rules! file_bench {
    ($name:ident, $fcn:ident) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use std::fs::read;
            let file: Vec<u8> = read("./data/words.txt").expect("cannot read words.txt");
            b.iter(|| black_box($fcn(&file)))
        }
    };
}

file_bench!(file_default, default);
file_bench!(file_djb2, djb2);
file_bench!(file_sdbm, sdbm);
file_bench!(file_loselose, loselose);
file_bench!(file_oaat, oaat);
file_bench!(file_lookup3, lookup3);
file_bench!(file_passthrough, passthrough);
file_bench!(file_fnv1a64, fnv1a64);
file_bench!(file_fnv1a32, fnv1a32);
file_bench!(file_spooky, spooky);
file_bench!(file_bricolage, bricolage);
