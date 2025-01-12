//! From http://burtleburtle.net/bob/hash/spooky.html
//!
//! Quoted comments are from http://burtleburtle.net/bob/c/SpookyV2.h or
//! http://burtleburtle.net/bob/c/SpookyV2.cpp

use core::{ mem, ptr};
use super::{rot64, const_slice_window};

/// number of uint64's in internal state
const SC_NUM_VARS: usize = 12;
/// size of the internal state in bytes
const SC_BLOCK_SIZE: usize = SC_NUM_VARS * mem::size_of::<u64>(); // 96
/// size of buffer of unhashed data, in bytes
const SC_BUF_SIZE: usize = 2 * SC_BLOCK_SIZE; // 192
/// > SC_CONST: a constant which:
/// >  * is not zero
/// >  * is odd
/// >  * is a not-very-regular mix of 1's and 0's
/// >  * does not need any other special mathematical properties
const SC_CONST: u64 = 0xdeadbeefdeadbeefu64;

/// > This is used if the input is 96 bytes long or longer.
/// >
/// > The internal state is fully overwritten every 96 bytes.
/// > Every input bit appears to cause at least 128 bits of entropy
/// > before 96 other bytes are combined, when run forward or backward
/// >   For every input bit,
/// >   Two inputs differing in just that input bit
/// >   Where "differ" means xor or subtraction
/// >   And the base value is random
/// >   When run forward or backwards one Mix
/// > I tried 3 pairs of each; they all differed by at least 212 bits.
///
/// data indices: 0..11
/// state indices: 0..11
#[inline(always)]
const fn mix(data: &[u64], state: &mut [u64; SC_NUM_VARS]) {
    debug_assert!(data.len() >= 12);
    state[0] = state[0].wrapping_add(data[0]);
    state[2] ^= state[10];
    state[11] ^= state[0];
    state[0] = rot64(state[0], 11);
    state[11] = state[11].wrapping_add(state[1]);
    state[1] = state[1].wrapping_add(data[1]);
    state[3] ^= state[11];
    state[0] ^= state[1];
    state[1] = rot64(state[1], 32);
    state[0] = state[0].wrapping_add(state[2]);
    state[2] = state[2].wrapping_add(data[2]);
    state[4] ^= state[0];
    state[1] ^= state[2];
    state[2] = rot64(state[2], 43);
    state[1] = state[1].wrapping_add(state[3]);
    state[3] = state[3].wrapping_add(data[3]);
    state[5] ^= state[1];
    state[2] ^= state[3];
    state[3] = rot64(state[3], 31);
    state[2] = state[2].wrapping_add(state[4]);
    state[4] = state[4].wrapping_add(data[4]);
    state[6] ^= state[2];
    state[3] ^= state[4];
    state[4] = rot64(state[4], 17);
    state[3] = state[3].wrapping_add(state[5]);
    state[5] = state[5].wrapping_add(data[5]);
    state[7] ^= state[3];
    state[4] ^= state[5];
    state[5] = rot64(state[5], 28);
    state[4] = state[4].wrapping_add(state[6]);
    state[6] = state[6].wrapping_add(data[6]);
    state[8] ^= state[4];
    state[5] ^= state[6];
    state[6] = rot64(state[6], 39);
    state[5] = state[5].wrapping_add(state[7]);
    state[7] = state[7].wrapping_add(data[7]);
    state[9] ^= state[5];
    state[6] ^= state[7];
    state[7] = rot64(state[7], 57);
    state[6] = state[6].wrapping_add(state[8]);
    state[8] = state[8].wrapping_add(data[8]);
    state[10] ^= state[6];
    state[7] ^= state[8];
    state[8] = rot64(state[8], 55);
    state[7] = state[7].wrapping_add(state[9]);
    state[9] = state[9].wrapping_add(data[9]);
    state[11] ^= state[7];
    state[8] ^= state[9];
    state[9] = rot64(state[9], 54);
    state[8] = state[8].wrapping_add(state[10]);
    state[10] = state[10].wrapping_add(data[10]);
    state[0] ^= state[8];
    state[9] ^= state[10];
    state[10] = rot64(state[10], 22);
    state[9] = state[9].wrapping_add(state[11]);
    state[11] = state[11].wrapping_add(data[11]);
    state[1] ^= state[9];
    state[10] ^= state[11];
    state[11] = rot64(state[11], 46);
    state[10] = state[10].wrapping_add(state[0]);
}

/// > Mix all 12 inputs together so that h0, h1 are a hash of
/// > them all.
/// >
/// > For two inputs differing in just the input bits Where
/// > "differ" means xor or subtraction And the base value is
/// > random, or a counting value starting at that bit The final
/// > result will have each bit of h0, h1 flip For every input
/// > bit, with probability 50 +- .3% For every pair of input
/// > bits, with probability 50 +- 3%
/// >
/// > This does not rely on the last Mix() call having already
/// > mixed some. Two iterations was almost good enough for a
/// > 64-bit result, but a 128-bit result is reported, so End()
/// > does three iterations.
#[inline(always)]
const fn end_partial(state: &mut [u64; SC_NUM_VARS]) {
    state[11] = state[11].wrapping_add(state[1]);
    state[2] ^= state[11];
    state[1] = rot64(state[1], 44);
    state[0] = state[0].wrapping_add(state[2]);
    state[3] ^= state[0];
    state[2] = rot64(state[2], 15);
    state[1] = state[1].wrapping_add(state[3]);
    state[4] ^= state[1];
    state[3] = rot64(state[3], 34);
    state[2] = state[2].wrapping_add(state[4]);
    state[5] ^= state[2];
    state[4] = rot64(state[4], 21);
    state[3] = state[3].wrapping_add(state[5]);
    state[6] ^= state[3];
    state[5] = rot64(state[5], 38);
    state[4] = state[4].wrapping_add(state[6]);
    state[7] ^= state[4];
    state[6] = rot64(state[6], 33);
    state[5] = state[5].wrapping_add(state[7]);
    state[8] ^= state[5];
    state[7] = rot64(state[7], 10);
    state[6] = state[6].wrapping_add(state[8]);
    state[9] ^= state[6];
    state[8] = rot64(state[8], 13);
    state[7] = state[7].wrapping_add(state[9]);
    state[10] ^= state[7];
    state[9] = rot64(state[9], 38);
    state[8] = state[8].wrapping_add(state[10]);
    state[11] ^= state[8];
    state[10] = rot64(state[10], 53);
    state[9] = state[9].wrapping_add(state[11]);
    state[0] ^= state[9];
    state[11] = rot64(state[11], 42);
    state[10] = state[10].wrapping_add(state[0]);
    state[1] ^= state[10];
    state[0] = rot64(state[0], 54);
}

#[inline(always)]
const fn end(data: &[u64; SC_NUM_VARS], state: &mut [u64; SC_NUM_VARS]) {
    state[0] = state[0].wrapping_add(data[0]);
    state[1] = state[1].wrapping_add(data[1]);
    state[2] = state[2].wrapping_add(data[2]);
    state[3] = state[3].wrapping_add(data[3]);
    state[4] = state[4].wrapping_add(data[4]);
    state[5] = state[5].wrapping_add(data[5]);
    state[6] = state[6].wrapping_add(data[6]);
    state[7] = state[7].wrapping_add(data[7]);
    state[8] = state[8].wrapping_add(data[8]);
    state[9] = state[9].wrapping_add(data[9]);
    state[10] = state[10].wrapping_add(data[10]);
    state[11] = state[11].wrapping_add(data[11]);
    end_partial(state);
    end_partial(state);
    end_partial(state);
}

/// > The goal is for each bit of the input to expand into 128
/// > bits of apparent entropy before it is fully overwritten. n
/// > trials both set and cleared at least m bits of h0 h1 h2 h3
/// >   n: 2   m: 29
/// >   n: 3   m: 46
/// >   n: 4   m: 57
/// >   n: 5   m: 107
/// >   n: 6   m: 146
/// >   n: 7   m: 152
/// > when run forwards or backwards for all 1-bit and 2-bit
/// > diffs with diffs defined by either xor or subtraction with
/// > a base of all zeros plus a counter, or plus another bit,
/// > or random

#[inline(always)]
const fn short_mix(h: &mut [u64; 4]) {
    h[2] = rot64(h[2], 50);
    h[2] = h[2].wrapping_add(h[3]);
    h[0] ^= h[2];
    h[3] = rot64(h[3], 52);
    h[3] = h[3].wrapping_add(h[0]);
    h[1] ^= h[3];
    h[0] = rot64(h[0], 30);
    h[0] = h[0].wrapping_add(h[1]);
    h[2] ^= h[0];
    h[1] = rot64(h[1], 41);
    h[1] = h[1].wrapping_add(h[2]);
    h[3] ^= h[1];
    h[2] = rot64(h[2], 54);
    h[2] = h[2].wrapping_add(h[3]);
    h[0] ^= h[2];
    h[3] = rot64(h[3], 48);
    h[3] = h[3].wrapping_add(h[0]);
    h[1] ^= h[3];
    h[0] = rot64(h[0], 38);
    h[0] = h[0].wrapping_add(h[1]);
    h[2] ^= h[0];
    h[1] = rot64(h[1], 37);
    h[1] = h[1].wrapping_add(h[2]);
    h[3] ^= h[1];
    h[2] = rot64(h[2], 62);
    h[2] = h[2].wrapping_add(h[3]);
    h[0] ^= h[2];
    h[3] = rot64(h[3], 34);
    h[3] = h[3].wrapping_add(h[0]);
    h[1] ^= h[3];
    h[0] = rot64(h[0], 5);
    h[0] = h[0].wrapping_add(h[1]);
    h[2] ^= h[0];
    h[1] = rot64(h[1], 36);
    h[1] = h[1].wrapping_add(h[2]);
    h[3] ^= h[1];
}

/// > Mix all 4 inputs together so that h0, h1 are a hash of them all.
/// >
/// > For two inputs differing in just the input bits
/// > Where "differ" means xor or subtraction
/// > And the base value is random, or a counting value starting at that bit
/// > The final result will have each bit of h0, h1 flip
/// > For every input bit,
/// > with probability 50 +- .3% (it is probably better than that)
/// > For every pair of input bits,
/// > with probability 50 +- .75% (the worst case is approximately that)

#[inline(always)]
const fn short_end(h: &mut [u64; 4]) {
    h[3] ^= h[2];
    h[2] = rot64(h[2], 15);
    h[3] = h[3].wrapping_add(h[2]);
    h[0] ^= h[3];
    h[3] = rot64(h[3], 52);
    h[0] = h[0].wrapping_add(h[3]);
    h[1] ^= h[0];
    h[0] = rot64(h[0], 26);
    h[1] = h[1].wrapping_add(h[0]);
    h[2] ^= h[1];
    h[1] = rot64(h[1], 51);
    h[2] = h[2].wrapping_add(h[1]);
    h[3] ^= h[2];
    h[2] = rot64(h[2], 28);
    h[3] = h[3].wrapping_add(h[2]);
    h[0] ^= h[3];
    h[3] = rot64(h[3], 9);
    h[0] = h[0].wrapping_add(h[3]);
    h[1] ^= h[0];
    h[0] = rot64(h[0], 47);
    h[1] = h[1].wrapping_add(h[0]);
    h[2] ^= h[1];
    h[1] = rot64(h[1], 54);
    h[2] = h[2].wrapping_add(h[1]);
    h[3] ^= h[2];
    h[2] = rot64(h[2], 32);
    h[3] = h[3].wrapping_add(h[2]);
    h[0] ^= h[3];
    h[3] = rot64(h[3], 25);
    h[0] = h[0].wrapping_add(h[3]);
    h[1] ^= h[0];
    h[0] = rot64(h[0], 63);
    h[1] = h[1].wrapping_add(h[0]);
}

/// > Short is used for messages under 192 bytes in length. Short
/// > has a low startup cost, the normal mode is good for long
/// > keys, the cost crossover is at about 192 bytes. The two modes
/// > were held to the same quality bar.
const fn short(message: &[u8], length: usize, hash1: &mut u64, hash2: &mut u64) {
    debug_assert!(length <= SC_BUF_SIZE);
    let mut h: [u64; 4] = [*hash1, *hash2, SC_CONST, SC_CONST];
    let mut bytes_cursor = const_slice_window(message, 0, message.len());

    while bytes_cursor.len() >= 4 * mem::size_of::<u64>() {
            let mut buf = [0u64; 4 * mem::size_of::<u64>()];
        let words: &[u64] = if (mem::align_of_val(bytes_cursor) & 3) == 0 {
            unsafe { mem::transmute::<&[u8], &[u64]>(const_slice_window(bytes_cursor, 0, 4*mem::size_of::<u64>())) }
        } else {
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes_cursor.as_ptr(),
                    &mut buf as *mut u64 as *mut u8,
                    4 * mem::size_of::<u64>(),
                );
            }
            &buf
        };
        h[2] = h[2].wrapping_add(words[0]);
        h[3] = h[3].wrapping_add(words[1]);
        short_mix(&mut h);
        h[0] = h[0].wrapping_add(words[2]);
        h[1] = h[1].wrapping_add(words[3]);
        bytes_cursor = const_slice_window(
            bytes_cursor,
            4 * mem::size_of::<u64>(),
            bytes_cursor.len() - 4 * mem::size_of::<u64>(),
        );
    }

    if bytes_cursor.len() >= 2 * mem::size_of::<u64>() {
        let mut buf = [0u64; 2 * mem::size_of::<u64>()];
        let words: &[u64] = if (mem::align_of_val(message) & 3) == 0 {
            unsafe { mem::transmute::<&[u8], &[u64]>(message) }
        } else {
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes_cursor.as_ptr(),
                    &mut buf as *mut u64 as *mut u8,
                    2 * mem::size_of::<u64>(),
                );
            }
            &buf
        };
        h[2] = h[2].wrapping_add(words[0]);
        h[3] = h[3].wrapping_add(words[1]);
        short_mix(&mut h);
    } else if ! bytes_cursor.is_empty() {
        h[3] = h[3].wrapping_add(length as u64).wrapping_shl(56);
        if bytes_cursor.len() >= 12 {
            if bytes_cursor.len() > 14 {
                h[3] = h[3].wrapping_add(bytes_cursor[14] as u64).wrapping_shl(48);
            }
            if bytes_cursor.len() > 13 {
                h[3] = h[3].wrapping_add(bytes_cursor[13] as u64).wrapping_shl(40);
            }
            if bytes_cursor.len() > 12 {
                h[3] = h[3].wrapping_add(bytes_cursor[12] as u64).wrapping_shl(32);
            }
            h[2] = h[2].wrapping_add(load_int_le!(bytes_cursor, 0, u64));
            h[3] = h[3].wrapping_add(load_int_le!(bytes_cursor, 8, u32) as u64);
        } else if bytes_cursor.len() >= 8 {
            if bytes_cursor.len() > 10 {
                h[3] = h[3].wrapping_add(bytes_cursor[10] as u64).wrapping_shl(16);
            }
            if bytes_cursor.len() > 9 {
                h[3] = h[3].wrapping_add(bytes_cursor[9] as u64).wrapping_shl(8);
            }
            if bytes_cursor.len() > 8 {
                h[3] = h[3].wrapping_add(bytes_cursor[8] as u64);
            }
            h[2] = h[2].wrapping_add(load_int_le!(bytes_cursor, 0, u64));
        } else if bytes_cursor.len() >= 4 {
            if bytes_cursor.len() > 6 {
                h[2] = h[2].wrapping_add(bytes_cursor[6] as u64).wrapping_shl(48);
            }
            if bytes_cursor.len() > 5 {
                h[2] = h[2].wrapping_add(bytes_cursor[5] as u64).wrapping_shl(40);
            }
            if bytes_cursor.len() > 4 {
                h[2] = h[2].wrapping_add(bytes_cursor[4] as u64).wrapping_shl(32);
            }
            h[2] = h[2].wrapping_add(load_int_le!(bytes_cursor, 0, u32) as u64);
        } else {
            if bytes_cursor.len() > 2 {
                h[2] = h[2].wrapping_add(bytes_cursor[2] as u64).wrapping_shl(16);
            }
            if bytes_cursor.len() > 1 {
                h[2] = h[2].wrapping_add(bytes_cursor[1] as u64).wrapping_shl(8);
            }
            h[2] = h[2].wrapping_add(bytes_cursor[0] as u64);
        } 
    }

    short_end(&mut h);
    *hash1 = h[0];
    *hash2 = h[1];
}

/// From http://burtleburtle.net/bob/hash/spooky.html
/// > SpookyHash is a public domain noncryptographic hash function producing well-distributed
/// > 128-bit hash values for byte arrays of any length. It can produce 64-bit and 32-bit hash values
/// > too, at the same speed, just use the bottom n bits. The C++ reference implementation is
/// > specific to 64-bit x86 platforms, in particular it assumes the processor is little endian. Long
/// > keys hash in 3 bytes per cycle, short keys take about 1 byte per cycle, and there is a 30 cycle
/// > startup cost. Keys can be supplied in fragments. The function allows a 128-bit seed. It's named
/// > SpookyHash because it was released on Halloween.
#[derive(PartialEq, Eq, Copy, Clone, Debug, PartialOrd, Ord)]
pub struct SpookyHasher {
    // unhashed data, for partial messages; 2 * m_state, in bytes
    pub m_data: [u8; SC_BUF_SIZE],
    // internal state of the hash
    pub m_state: [u64; SC_NUM_VARS],
    // total length of the input so far
    pub m_length: usize,
    // length of unhashed data stashed in m_data
    pub m_remainder: usize,
}

impl SpookyHasher {
    pub const fn default() -> SpookyHasher {
        SpookyHasher {
            m_data: [0; SC_BUF_SIZE],
            m_state: [0; SC_NUM_VARS],
            m_length: 0,
            m_remainder: 0,
        }
    }
    pub const fn new(seed1: u64, seed2: u64) -> SpookyHasher {
        let mut sh = Self::default();
        sh.m_state[0] = seed1;
        sh.m_state[3] = seed1;
        sh.m_state[6] = seed1;
        sh.m_state[9] = seed1;
        sh.m_state[1] = seed2;
        sh.m_state[4] = seed2;
        sh.m_state[7] = seed2;
        sh.m_state[10] = seed2;
        sh.m_state[2] = SC_CONST;
        sh.m_state[5] = SC_CONST;
        sh.m_state[8] = SC_CONST;
        sh.m_state[11] = SC_CONST;
        sh
    }

    pub const fn finish128(&self) -> (u64, u64) {
        if self.m_length < SC_BUF_SIZE {
            let mut hash1 = self.m_state[0];
            let mut hash2 = self.m_state[1];
            short(&self.m_data, self.m_length, &mut hash1, &mut hash2);
            return (hash1, hash2);
        }
        let mut state = self.m_state;
        let mut remainder = self.m_remainder;
        let mut processed = 0;
        if self.m_remainder >= SC_BLOCK_SIZE {
            let data = unsafe { mem::transmute::<&[u8], &[u64]>(&self.m_data) };
            mix(data, &mut state);
            processed = SC_BLOCK_SIZE;
            remainder -= SC_BLOCK_SIZE;
        }
        let mut data = [0u64; SC_NUM_VARS];
        unsafe {
            ptr::copy_nonoverlapping::<u8>(
                (&self.m_data as *const u8).add(processed),
                data.as_mut_ptr() as *mut u8,
                remainder,
            );
            ptr::write_bytes(
                (data.as_mut_ptr() as *mut u8).add(SC_BLOCK_SIZE - 1),
                remainder as u8,
                1,
            );
        }
        end(&data, &mut state);
        (state[0], state[1])
    }

    #[inline(always)]
    pub const fn finish(&self) -> u64 {
        self.finish128().0
    }

    pub const fn write(&mut self, bytes: &[u8]) {
        let new_length = self.m_remainder + bytes.len();
        // if the fragment is too short, store it for later
        if new_length < SC_BUF_SIZE {
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    self.m_data.as_mut_ptr().add(self.m_remainder ),
                    bytes.len(),
                );
            }
            self.m_length += bytes.len();
            self.m_remainder = new_length;
            return;
        }
        self.m_length += bytes.len();
        let mut processed = 0;
        // if we've got anything stuffed away, use it now
        if self.m_remainder > 0 {
            // add the prefix of bytes to m_data
            processed = SC_BUF_SIZE - self.m_remainder;
            unsafe {
                ptr::copy_nonoverlapping(
                    bytes.as_ptr(),
                    self.m_data.as_mut_ptr().add(self.m_remainder ),
                    processed,
                );
            }
            let data: &[u64] = unsafe { mem::transmute::<&[u8], &[u64]>(&self.m_data) };
            mix(data, &mut self.m_state);
            mix(const_slice_window(data, SC_NUM_VARS, data.len() - SC_NUM_VARS), &mut self.m_state);
            self.m_remainder = 0;
        }
        // process the rest of the bytes
        let mut bytes_cursor = const_slice_window(bytes, processed, bytes.len() - processed);
        while bytes_cursor.len() >= SC_BLOCK_SIZE {
                // handle whole blocks of SC_BLOCK_SIZE bytes
                if (mem::align_of_val(bytes_cursor) & 7) == 0 {
                    let data: &[u64] = unsafe { mem::transmute::<&[u8], &[u64]>(bytes_cursor) };
                    mix(data, &mut self.m_state);
                } else {
                    unsafe {
                        ptr::copy_nonoverlapping(
                            bytes_cursor.as_ptr(),
                            self.m_data.as_mut_ptr(),
                            SC_BLOCK_SIZE,
                        );
                    }
                    let  data: &[u64] = unsafe { mem::transmute::<&[u8], &[u64]>(&self.m_data) };
                    mix(data, &mut self.m_state);
            }
                    bytes_cursor = const_slice_window(bytes_cursor, SC_BLOCK_SIZE, bytes_cursor.len() - SC_BLOCK_SIZE);
        }
        if ! bytes_cursor.is_empty() {
                // stuff away the last few bytes
                unsafe {
                    ptr::copy_nonoverlapping(
                        bytes_cursor.as_ptr(),
                        self.m_data.as_mut_ptr(),
                        bytes_cursor.len(),
                    );
                }
                self.m_remainder = bytes_cursor.len();
            
    }
    }
}

duplicate_const_traits!(SpookyHasher);
hasher_to_fcn!(
    /// Provide access to Lookup3Hasher in a single call.
    spooky,
    SpookyHasher
);

#[cfg(test)]
mod spookyhash_test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(spooky(b""), 13905049616862401802);
        assert_eq!(spooky(b"a"), 16932945748884078726);
        assert_eq!(spooky(b"b"), 5781063613870495197);
        assert_eq!(spooky(b"ab"), 13849109452443161137);
        assert_eq!(spooky(b"abcd"), 4142038200967391753);
        assert_eq!(spooky(b"abcdefg"), 2761526316938866980);
        assert_eq!(spooky(b"abcdefghijklmnopqrstuvwxyz"), 16192181224158463141);
    }
}
