pub fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut ret = Vec::new();
    for i in 0..a.len() {
        ret.push(a[i] ^ b[i]);
    }
    ret
}