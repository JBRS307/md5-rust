use std::sync::OnceLock;

pub fn sine_const(i: usize) -> u32 {
    static SINE_CONSTS: OnceLock<[u32; 64]> = OnceLock::new();
    let consts = SINE_CONSTS.get_or_init(|| {
        let mut sines: [u32; 64] = [0; 64];
        for (i, sine_ref) in sines.iter_mut().enumerate() {
            let sine = f64::abs(f64::sin((i + 1) as f64));
            let val = (sine * (1u64 << 32) as f64).floor() as u32;
            *sine_ref = val;
        }
        sines
    });
    consts[i]
}

pub fn index_round1(i: usize) -> usize {
    i
}

pub fn index_round2(i: usize) -> usize {
    (i * 5 + 1) % 16
}

pub fn index_round3(i: usize) -> usize {
    (3 * i + 5) % 16
}

pub fn index_round4(i: usize) -> usize {
    (7 * i) % 16
}

pub fn rotate_values(round: usize) -> [u32; 4] {
    static ROTATE_VALUES: OnceLock<[[u32; 4]; 4]> = OnceLock::new();
    let values = ROTATE_VALUES.get_or_init(|| {
        [
            [7, 12, 17, 22],
            [5, 9, 14, 20],
            [4, 11, 16, 23],
            [6, 10, 15, 21],
        ]
    });
    values[round - 1]
}
