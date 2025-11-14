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
        let mut a = A;
        let mut b = B;
        let mut c = C;
        let mut d = D;
        for i in 0..blocks.len() {
            let block: [u64; 16] = blocks[i]
                .iter()
                .map(|elem| *elem as u64)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            let aa = A;
            let bb = B;
            let cc = C;
            let dd = D;
        }
    }

    fn round1(&mut self) {}
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
