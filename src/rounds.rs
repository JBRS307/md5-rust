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
        for i in 0..blocks.len() {
            let block: [u64; 16] = blocks[i]
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

fn ff(a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64], k: usize, s: u64, t: u64) {
    let temp = (*a + f(*b, *c, *d) + block[k] + t) & (u32::MAX as u64);
    let temp = rotate_left(temp, s);
    *a = (*b + temp) & (u32::MAX as u64);
}
