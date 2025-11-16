mod consts;
mod rounds;

use crate::rounds::Md5Hash;

fn main() {
    let msg = "aaaa";
    println!("{msg}");
    let msg_bytes = msg.as_bytes();
    let hash = Md5Hash::hash(msg_bytes);
    println!("{:?}", hash.bytes());
    println!("{}", hash.hex_digest());

    let msg = "a".repeat(70);
    println!("{msg}");
    let msg_bytes = msg.as_bytes();
    let hash = Md5Hash::hash(msg_bytes);
    println!("{:?}", hash.bytes());
    println!("{}", hash.hex_digest());
}
