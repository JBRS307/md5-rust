// S consts

use std::sync::OnceLock;

pub fn sine_const(i: usize) -> u64 {
    static SINE_CONSTS: OnceLock<[u64; 64]> = OnceLock::new();
    let consts = SINE_CONSTS.get_or_init(|| {
        let mut sines: [u64; 64] = [0; 64];
        for i in 0..64 {
            let sine = f64::sin((i + 1) as f64).floor() as u64;
            let val = (sine * (1 << 32)) & (u32::MAX as u64);
            sines[i] = val;
        }
        sines
    });
    consts[i]
}

fn index_round1(i: usize) -> usize {
    i
}

fn index_round2(i: usize) -> usize {
    (i * 5 + 1) % 16
}

fn index_round3(i: usize) -> usize {
    (3 * i + 5) % 16
}

fn index_round4(i: usize) -> usize {
    (7 * i) % 16
}

pub fn index_function(round: usize) -> fn(usize) -> usize {
    static INDEX_FUNCTIONS: OnceLock<[fn(usize) -> usize; 4]> = OnceLock::new();
    let functions =
        INDEX_FUNCTIONS.get_or_init(|| [index_round1, index_round2, index_round3, index_round4]);
    functions[round]
}

pub fn rotate_value(round: usize, i: usize) -> u64 {
    static ROTATE_VALUES: OnceLock<[[u64; 4]; 4]> = OnceLock::new();
    let values = ROTATE_VALUES.get_or_init(|| {
        [
            [7, 12, 17, 22],
            [5, 9, 14, 20],
            [4, 11, 16, 23],
            [6, 10, 15, 21],
        ]
    });
    values[round][i]
}
