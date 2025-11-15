use crate::sine_consts::*;

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

pub struct Md5Hash {
    bytes: Vec<u8>,
    hex_digest: String,
}

impl Md5Hash {
    pub fn hash(msg_bytes: &[u8]) -> Self {
        let bytes = pad_message(msg_bytes.to_vec());
        let blocks = split_to_blocks(bytes);
        let hashed_bytes = process_msg(blocks);

        let hex_digest = hex_digest(&hashed_bytes);
        Self {
            bytes: hashed_bytes,
            hex_digest,
        }
    }

    pub fn hex_digest(&self) -> &str {
        &self.hex_digest
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
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
    bytes.extend((original_length * 8).to_le_bytes());
    bytes
}

fn split_to_blocks(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    assert_eq!(bytes.len() % 64, 0);

    let mut blocks = vec![];
    for i in (0..bytes.len()).step_by(64) {
        let mut block = vec![];
        for j in (i..i + 64).step_by(4) {
            let word = u32::from_le_bytes([bytes[j], bytes[j + 1], bytes[j + 2], bytes[j + 3]]);
            block.push(word);
        }
        blocks.push(block.try_into().unwrap());
    }
    blocks
}

fn process_msg(blocks: Vec<[u32; 16]>) -> Vec<u8> {
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
        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        round_n(1, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(2, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(3, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(4, &mut a, &mut b, &mut c, &mut d, &block);

        a = (a + aa) & (u32::MAX as u64);
        b = (b + bb) & (u32::MAX as u64);
        c = (c + cc) & (u32::MAX as u64);
        d = (d + dd) & (u32::MAX as u64);
    }
    let a = a as u32;
    let b = b as u32;
    let c = c as u32;
    let d = d as u32;

    let mut output = vec![];
    output.extend(a.to_le_bytes());
    output.extend(b.to_le_bytes());
    output.extend(c.to_le_bytes());
    output.extend(d.to_le_bytes());

    output
}

fn rotate_left(x: u64, s: u64) -> u64 {
    ((x << s) | (x >> (32 - s))) & (u32::MAX as u64)
}

fn f(x: u64, y: u64, z: u64) -> u64 {
    const FULL: u64 = u32::MAX as u64;
    (x & y) | ((!x & FULL) & z)
}

fn g(x: u64, y: u64, z: u64) -> u64 {
    const FULL: u64 = u32::MAX as u64;
    (x & z) | (y & (!z & FULL))
}

fn h(x: u64, y: u64, z: u64) -> u64 {
    x ^ y ^ z
}

fn i(x: u64, y: u64, z: u64) -> u64 {
    const FULL: u64 = u32::MAX as u64;
    y ^ (x | (!z & FULL))
}

#[allow(clippy::too_many_arguments)]
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

fn round_n(round: usize, a: &mut u64, b: &mut u64, c: &mut u64, d: &mut u64, block: &[u64]) {
    let idx = match round {
        1 => index_round1,
        2 => index_round2,
        3 => index_round3,
        4 => index_round4,
        _ => unreachable!(),
    };
    let logic = match round {
        1 => f,
        2 => g,
        3 => h,
        4 => i,
        _ => unreachable!(),
    };

    let s_vals = rotate_values(round);
    for i in 0..16 {
        let k = idx(i);
        let s = s_vals[i % 4];
        let t = sine_const(i + 16 * (round - 1));
        match i % 4 {
            0 => md5_round(a, b, c, d, block, k, s, t, logic),
            1 => md5_round(d, a, b, c, block, k, s, t, logic),
            2 => md5_round(c, d, a, b, block, k, s, t, logic),
            3 => md5_round(b, c, d, a, block, k, s, t, logic),
            _ => unreachable!(),
        }
    }
}

fn hex_digest(bytes: &[u8]) -> String {
    let mut digest = String::new();
    for byte in bytes {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
