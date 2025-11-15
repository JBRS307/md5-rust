mod rounds;
mod sine_consts;

use crate::rounds::Md5Hash;

fn main() {
    let test1 = "a";
    let test1_bytes = test1.as_bytes();
    let hash1 = Md5Hash::hash(test1_bytes);
    _ = hash1.bytes();
    println!("{}", hash1.hex_digest());
}
