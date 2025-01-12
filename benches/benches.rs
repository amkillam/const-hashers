#![feature(test)]

extern crate test;
use core::hash::Hasher;
use test::{Bencher, black_box};

macro_rules! tiny_bench {
    ($name:ident, $($fcn:tt)*) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| black_box($($fcn)*(b"abcd")))
        }
    };
}

#[cfg(feature = "builtin")]
tiny_bench!(tiny_default, const_hashers::builtin::default);
#[cfg(feature = "oz")]
tiny_bench!(tiny_sdbm, const_hashers::oz::sdbm);
#[cfg(feature = "oz")]
tiny_bench!(tiny_loselose, const_hashers::oz::loselose);
#[cfg(feature = "oz")]
tiny_bench!(tiny_bricolage, const_hashers::pigeon::bricolage);
#[cfg(feature = "oz")]
tiny_bench!(tiny_djb2, const_hashers::oz::djb2);
#[cfg(feature = "fnv")]
tiny_bench!(tiny_fnv1a64, const_hashers::fnv::fnv1a64);
#[cfg(feature = "jenkins")]
#[cfg(feature = "jenkins")]
tiny_bench!(tiny_lookup3, const_hashers::jenkins::lookup3);
#[cfg(feature = "jenkins")]
tiny_bench!(tiny_spooky, const_hashers::jenkins::spooky_hash::spooky);
#[cfg(feature = "jenkins")]
tiny_bench!(tiny_oaat, const_hashers::jenkins::oaat);
#[cfg(feature = "null")]
tiny_bench!(tiny_passthrough, const_hashers::passthrough);

macro_rules! w32_bench {
    ($name:ident, $count:expr, $($hasher:tt)*) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut h = $($hasher)*::default();
                for i in 0..$count {
                    h.write_i32(i);
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
w32_bench!(w32_10_default, 10, const_hashers::builtin::DefaultHasher);
#[cfg(feature = "oz")]
w32_bench!(w32_10_djb2, 10, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w32_bench!(w32_10_sdbm, 10, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w32_bench!(w32_10_loselose, 10, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w32_bench!(w32_10_oaat, 10, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w32_bench!(w32_10_lookup3, 10, const_hashers::jenkins::Lookup3Hasher);
#[cfg(feature = "null")]
w32_bench!(
    w32_10_passthrough,
    10,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w32_bench!(w32_10_fnv1a64, 10, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w32_bench!(
    w32_10_spooky,
    10,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w32_bench!(w32_10_bricolage, 10, const_hashers::pigeon::Bricolage);

#[cfg(feature = "builtin")]
w32_bench!(w32_100_default, 100, const_hashers::builtin::DefaultHasher);
#[cfg(feature = "oz")]
w32_bench!(w32_100_djb2, 100, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w32_bench!(w32_100_sdbm, 100, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w32_bench!(w32_100_loselose, 100, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w32_bench!(w32_100_oaat, 100, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w32_bench!(w32_100_lookup3, 100, const_hashers::jenkins::Lookup3Hasher);
#[cfg(feature = "null")]
w32_bench!(
    w32_100_passthrough,
    100,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w32_bench!(w32_100_fnv1a64, 100, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w32_bench!(
    w32_100_spooky,
    100,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w32_bench!(w32_100_bricolage, 100, const_hashers::pigeon::Bricolage);

#[cfg(feature = "builtin")]
w32_bench!(
    w32_1000_default,
    1000,
    const_hashers::builtin::DefaultHasher
);
#[cfg(feature = "oz")]
w32_bench!(w32_1000_djb2, 1000, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w32_bench!(w32_1000_sdbm, 1000, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w32_bench!(w32_1000_loselose, 1000, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w32_bench!(w32_1000_oaat, 1000, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w32_bench!(
    w32_1000_lookup3,
    1000,
    const_hashers::jenkins::Lookup3Hasher
);
#[cfg(feature = "null")]
w32_bench!(
    w32_1000_passthrough,
    1000,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w32_bench!(w32_1000_fnv1a64, 1000, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w32_bench!(
    w32_1000_spooky,
    1000,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w32_bench!(w32_1000_bricolage, 1000, const_hashers::pigeon::Bricolage);

macro_rules! w64_bench {
    ($name:ident, $count:expr, $($hasher:tt)*) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            b.iter(|| {
                let mut h = $($hasher)*::default();
                for i in 0..$count {
                    h.write_i64(i);
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
w64_bench!(w64_10_default, 10, const_hashers::builtin::DefaultHasher);
#[cfg(feature = "oz")]
w64_bench!(w64_10_djb2, 10, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w64_bench!(w64_10_sdbm, 10, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w64_bench!(w64_10_loselose, 10, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w64_bench!(w64_10_oaat, 10, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w64_bench!(w64_10_lookup3, 10, const_hashers::jenkins::Lookup3Hasher);
#[cfg(feature = "null")]
w64_bench!(
    w64_10_passthrough,
    10,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w64_bench!(w64_10_fnv1a64, 10, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w64_bench!(
    w64_10_spooky,
    10,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w64_bench!(w64_10_bricolage, 10, const_hashers::pigeon::Bricolage);

#[cfg(feature = "builtin")]
w64_bench!(w64_100_default, 100, const_hashers::builtin::DefaultHasher);
#[cfg(feature = "oz")]
w64_bench!(w64_100_djb2, 100, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w64_bench!(w64_100_sdbm, 100, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w64_bench!(w64_100_loselose, 100, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w64_bench!(w64_100_oaat, 100, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w64_bench!(w64_100_lookup3, 100, const_hashers::jenkins::Lookup3Hasher);
#[cfg(feature = "null")]
w64_bench!(
    w64_100_passthrough,
    100,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w64_bench!(w64_100_fnv1a64, 100, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w64_bench!(
    w64_100_spooky,
    100,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w64_bench!(w64_100_bricolage, 100, const_hashers::pigeon::Bricolage);

#[cfg(feature = "builtin")]
w64_bench!(
    w64_1000_default,
    1000,
    const_hashers::builtin::DefaultHasher
);
#[cfg(feature = "oz")]
w64_bench!(w64_1000_djb2, 1000, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
w64_bench!(w64_1000_sdbm, 1000, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
w64_bench!(w64_1000_loselose, 1000, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
w64_bench!(w64_1000_oaat, 1000, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
w64_bench!(
    w64_1000_lookup3,
    1000,
    const_hashers::jenkins::Lookup3Hasher
);
#[cfg(feature = "null")]
w64_bench!(
    w64_1000_passthrough,
    1000,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
w64_bench!(w64_1000_fnv1a64, 1000, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
w64_bench!(
    w64_1000_spooky,
    1000,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
w64_bench!(w64_1000_bricolage, 1000, const_hashers::pigeon::Bricolage);

fn read_words() -> Vec<String> {
    use std::fs::File;
    use std::io::BufReader;
    use std::io::prelude::*;

    let file = File::open("./data/words.txt").expect("cannot open words.txt");
    return BufReader::new(file)
        .lines()
        .map(|l| l.expect("bad read"))
        .collect();
}

macro_rules! words_bench {
    ($name:ident, $count:expr, $($hasher:tt)*) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            let words = read_words();
            b.iter(|| {
                let mut h = $($hasher)*::default();
                for i in words.iter().take($count) {
                    h.write(i.as_bytes());
                }
                black_box(h.finish())
            })
        }
    };
}

#[cfg(feature = "builtin")]
words_bench!(
    words1000_default,
    1000,
    const_hashers::builtin::DefaultHasher
);
#[cfg(feature = "oz")]
words_bench!(words1000_djb2, 1000, const_hashers::oz::DJB2Hasher);
#[cfg(feature = "oz")]
words_bench!(words1000_sdbm, 1000, const_hashers::oz::SDBMHasher);
#[cfg(feature = "oz")]
words_bench!(words1000_loselose, 1000, const_hashers::oz::LoseLoseHasher);
#[cfg(feature = "jenkins")]
words_bench!(words1000_oaat, 1000, const_hashers::jenkins::OAATHasher);
#[cfg(feature = "jenkins")]
words_bench!(
    words1000_lookup3,
    1000,
    const_hashers::jenkins::Lookup3Hasher
);
#[cfg(feature = "null")]
words_bench!(
    words1000_passthrough,
    1000,
    const_hashers::null::PassThroughHasher
);
#[cfg(feature = "fnv")]
words_bench!(words1000_fnv1a64, 1000, const_hashers::fnv::FNV1aHasher64);
#[cfg(feature = "jenkins")]
words_bench!(
    words1000_spooky,
    1000,
    const_hashers::jenkins::spooky_hash::SpookyHasher
);
#[cfg(feature = "oz")]
words_bench!(words1000_bricolage, 1000, const_hashers::pigeon::Bricolage);

macro_rules! file_bench {
    ($name:ident, $($fcn:tt)*) => {
        #[bench]
        fn $name(b: &mut Bencher) {
            use std::fs::read;
            let file: Vec<u8> = read("./data/words.txt").expect("cannot read words.txt");
            b.iter(|| black_box($($fcn)*(&file)))
        }
    };
}

#[cfg(feature = "builtin")]
file_bench!(file_default, const_hashers::builtin::default);
#[cfg(feature = "oz")]
file_bench!(file_djb2, const_hashers::oz::djb2);
#[cfg(feature = "oz")]
file_bench!(file_sdbm, const_hashers::oz::sdbm);
#[cfg(feature = "oz")]
file_bench!(file_loselose, const_hashers::oz::loselose);
#[cfg(feature = "jenkins")]
file_bench!(file_oaat, const_hashers::jenkins::oaat);
#[cfg(feature = "jenkins")]
file_bench!(file_lookup3, const_hashers::jenkins::lookup3);
#[cfg(feature = "null")]
file_bench!(file_passthrough, const_hashers::null::passthrough);
#[cfg(feature = "fnv")]
file_bench!(file_fnv1a64, const_hashers::fnv::fnv1a64);
#[cfg(feature = "fnv")]
file_bench!(file_fnv1a32, const_hashers::fnv::fnv1a32);
#[cfg(feature = "jenkins")]
file_bench!(file_spooky, const_hashers::jenkins::spooky_hash::spooky);
#[cfg(feature = "oz")]
file_bench!(file_bricolage, const_hashers::pigeon::bricolage);
