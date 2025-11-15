use std::u32;

use crate::sine_consts::*;

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

pub struct Md5Hash {
    bytes: Option<Vec<u8>>,
    hex_digest: Option<String>,
}

impl Md5Hash {
    pub fn hash(msg_bytes: &[u8]) -> Self {
        let bytes = pad_message(msg_bytes.to_vec());
        let blocks = split_to_blocks(bytes);

        let mut md5 = Self {
            bytes: None,
            hex_digest: None,
        };

        todo!()
    }

    fn process_msg(&mut self, blocks: Vec<[u8; 16]>) {
        let mut a = A as u64;
        let mut b = B as u64;
        let mut c = C as u64;
        let mut d = D as u64;
        for block in blocks {
            let block: [u64; 16] = block
                .iter()
                .map(|elem| *elem as u64)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let aa = A as u64;
            let bb = B as u64;
            let cc = C as u64;
            let dd = D as u64;

            // Round 1
            ff(&mut a, &mut b, &mut c, &mut d, &block, 0, S11, FF11);
        }
    }
}

fn pad_message(mut bytes: Vec<u8>) -> Vec<u8> {
    let original_length = bytes.len() as u64;
    if bytes.len() % 64 != 56 {
        bytes.push(0x80);
        while bytes.len() % 64 != 56 {
            bytes.push(0);
        }
    }
    for i in (0..8 as u64).rev() {
        const FULL: u64 = 0xff;
        let mask = FULL << (8 * i);
        let byte = ((mask & original_length) >> (8 * i)) as u8;
        bytes.push(byte);
    }
    bytes
}

fn split_to_blocks(bytes: Vec<u8>) -> Vec<[u8; 16]> {
    assert_eq!(bytes.len() % 16, 0);

    let mut blocks = vec![];
    for i in (0..bytes.len()).step_by(16) {
        let block: [u8; 16] = bytes[i..i + 16].try_into().unwrap();
        blocks.push(block);
    }
    blocks
}

fn rotate_left(x: u64, s: u64) -> u64 {
    ((x << s) | (x >> (32 - s))) & (u32::MAX as u64)
}

fn f(x: u64, y: u64, z: u64) -> u64 {
    (x & y) | (!x & z)
}

fn g(x: u64, y: u64, z: u64) -> u64 {
    (x & z) | (y & !z)
}

fn h(x: u64, y: u64, z: u64) -> u64 {
    x ^ y ^ z
}

fn i(x: u64, y: u64, z: u64) -> u64 {
    y ^ (x | !z)
}

fn md5_round(
    a: &mut u64,
    b: &mut u64,
    c: &mut u64,
    d: &mut u64,
    block: &[u64],
    k: usize,
    s: u64,
    t: u64,
    logic: fn(u64, u64, u64) -> u64,
) {
    let temp = (*a + logic(*b, *c, *d) + block[k] + t) & (u32::MAX as u64);
    let temp = rotate_left(temp, s);
    *a = (*b + temp) & (u32::MAX as u64);
}

fn round1(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64]) {
    md5_round(a, b, c, d, block, 0, S11, FF11, f);
    md5_round(a, b, c, d, block, 1, S12, FF12, f);
    md5_round(a, b, c, d, block, 2, S13, FF13, f);
    md5_round(a, b, c, d, block, 3, S14, FF14, f);
    md5_round(a, b, c, d, block, 4, S21, FF21, f);
    md5_round(a, b, c, d, block, 5, S22, FF22, f);
    md5_round(a, b, c, d, block, 6, S23, FF23, f);
    md5_round(a, b, c, d, block, 7, S24, FF24, f);
    md5_round(a, b, c, d, block, 8, S31, FF31, f);
    md5_round(a, b, c, d, block, 9, S32, FF32, f);
    md5_round(a, b, c, d, block, 10, S33, FF33, f);
    md5_round(a, b, c, d, block, 11, S34, FF34, f);
    md5_round(a, b, c, d, block, 12, S41, FF41, f);
    md5_round(a, b, c, d, block, 13, S42, FF42, f);
    md5_round(a, b, c, d, block, 14, S43, FF43, f);
    md5_round(a, b, c, d, block, 15, S44, FF44, f);
}

fn round2(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64]) {
    md5_round(a, b, c, d, block, 0, S11, GG11, g);
    md5_round(a, b, c, d, block, 1, S12, GG12, g);
    md5_round(a, b, c, d, block, 2, S13, GG13, g);
    md5_round(a, b, c, d, block, 3, S14, GG14, g);
    md5_round(a, b, c, d, block, 4, S21, GG21, g);
    md5_round(a, b, c, d, block, 5, S22, GG22, g);
    md5_round(a, b, c, d, block, 6, S23, GG23, g);
    md5_round(a, b, c, d, block, 7, S24, GG24, g);
    md5_round(a, b, c, d, block, 8, S31, GG31, g);
    md5_round(a, b, c, d, block, 9, S32, GG32, g);
    md5_round(a, b, c, d, block, 10, S33, GG33, g);
    md5_round(a, b, c, d, block, 11, S34, GG34, g);
    md5_round(a, b, c, d, block, 12, S41, GG41, g);
    md5_round(a, b, c, d, block, 13, S42, GG42, g);
    md5_round(a, b, c, d, block, 14, S43, GG43, g);
    md5_round(a, b, c, d, block, 15, S44, GG44, g);
}

fn round3(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64]) {
    md5_round(a, b, c, d, block, 0, S11, HH11, h);
    md5_round(a, b, c, d, block, 1, S12, HH12, h);
    md5_round(a, b, c, d, block, 2, S13, HH13, h);
    md5_round(a, b, c, d, block, 3, S14, HH14, h);
    md5_round(a, b, c, d, block, 4, S21, HH21, h);
    md5_round(a, b, c, d, block, 5, S22, HH22, h);
    md5_round(a, b, c, d, block, 6, S23, HH23, h);
    md5_round(a, b, c, d, block, 7, S24, HH24, h);
    md5_round(a, b, c, d, block, 8, S31, HH31, h);
    md5_round(a, b, c, d, block, 9, S32, HH32, h);
    md5_round(a, b, c, d, block, 10, S33, HH33, h);
    md5_round(a, b, c, d, block, 11, S34, HH34, h);
    md5_round(a, b, c, d, block, 12, S41, HH41, h);
    md5_round(a, b, c, d, block, 13, S42, HH42, h);
    md5_round(a, b, c, d, block, 14, S43, HH43, h);
    md5_round(a, b, c, d, block, 15, S44, HH44, h);
}

fn round4(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64]) {
    md5_round(a, b, c, d, block, 0, S11, II11, i);
    md5_round(a, b, c, d, block, 1, S12, II12, i);
    md5_round(a, b, c, d, block, 2, S13, II13, i);
    md5_round(a, b, c, d, block, 3, S14, II14, i);
    md5_round(a, b, c, d, block, 4, S21, II21, i);
    md5_round(a, b, c, d, block, 5, S22, II22, i);
    md5_round(a, b, c, d, block, 6, S23, II23, i);
    md5_round(a, b, c, d, block, 7, S24, II24, i);
    md5_round(a, b, c, d, block, 8, S31, II31, i);
    md5_round(a, b, c, d, block, 9, S32, II32, i);
    md5_round(a, b, c, d, block, 10, S33, II33, i);
    md5_round(a, b, c, d, block, 11, S34, II34, i);
    md5_round(a, b, c, d, block, 12, S41, II41, i);
    md5_round(a, b, c, d, block, 13, S42, II42, i);
    md5_round(a, b, c, d, block, 14, S43, II43, i);
    md5_round(a, b, c, d, block, 15, S44, II44, i);
}
