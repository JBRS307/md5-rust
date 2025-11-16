use crate::consts::*;

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

pub struct Md5Hash {
    bytes: [u8; 16],
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
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }
    bytes.extend((original_length * 8).to_le_bytes());
    bytes
}

fn split_to_blocks(bytes: Vec<u8>) -> Vec<[u32; 16]> {
    assert_eq!(bytes.len() % 64, 0);

    let mut blocks = vec![];
    for i in (0..bytes.len()).step_by(64) {
        let mut block: [u32; 16] = [0; 16];
        for (k, j) in (i..i + 64).step_by(4).enumerate() {
            let word = u32::from_le_bytes([bytes[j], bytes[j + 1], bytes[j + 2], bytes[j + 3]]);
            block[k] = word
        }
        blocks.push(block);
    }
    blocks
}

fn process_msg(blocks: Vec<[u32; 16]>) -> [u8; 16] {
    let mut a = A;
    let mut b = B;
    let mut c = C;
    let mut d = D;
    for block in blocks {
        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        round_n(1, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(2, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(3, &mut a, &mut b, &mut c, &mut d, &block);
        round_n(4, &mut a, &mut b, &mut c, &mut d, &block);

        a = u32::overflowing_add(a, aa).0;
        b = u32::overflowing_add(b, bb).0;
        c = u32::overflowing_add(c, cc).0;
        d = u32::overflowing_add(d, dd).0;
    }

    let mut output = [0u8; 16];
    let a_bytes: [u8; 4] = a.to_le_bytes();
    let b_bytes: [u8; 4] = b.to_le_bytes();
    let c_bytes: [u8; 4] = c.to_le_bytes();
    let d_bytes: [u8; 4] = d.to_le_bytes();

    output[0..4].copy_from_slice(&a_bytes);
    output[4..8].copy_from_slice(&b_bytes);
    output[8..12].copy_from_slice(&c_bytes);
    output[12..16].copy_from_slice(&d_bytes);
    output
}

fn rotate_left(x: u32, s: u32) -> u32 {
    (x << s) | (x >> (32 - s))
}

fn f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    (x & z) | (y & !z)
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

#[allow(clippy::too_many_arguments)]
fn md5_round(
    a: &mut u32,
    b: &mut u32,
    c: &mut u32,
    d: &mut u32,
    block: &[u32],
    k: usize,
    s: u32,
    t: u32,
    logic: fn(u32, u32, u32) -> u32,
) {
    let temp = u32::overflowing_add(*a, logic(*b, *c, *d))
        .0
        .overflowing_add(block[k])
        .0
        .overflowing_add(t)
        .0;
    let temp = rotate_left(temp, s);
    *a = u32::overflowing_add(*b, temp).0;
}

fn round_n(round: usize, a: &mut u32, b: &mut u32, c: &mut u32, d: &mut u32, block: &[u32]) {
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
    let mut digest = String::with_capacity(32);
    for byte in bytes {
        digest.push_str(&format!("{byte:02x}"));
    }
    digest
}
