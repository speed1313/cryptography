use rand::Rng;

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut b = base % modulus;
    let mut e = exponent;
    // b^e = b^(2^0 * e_0 + 2^1 * e_1 + ... + 2^n * e_n) = b^(2^0 * e_0) * b^(2^1 * e_1) * ... * b^(2^n * e_n)
    while e > 0 {
        // eの最下位ビットが1ならば、bを掛ける
        if e % 2 == 1 {
            result = (result * b) % modulus;
        }
        // eを1ビット右シフトする
        e >>= 1;
        // bを2乗する
        b = (b * b) % modulus;
    }
    result
}

fn main() {
    // a = r^x mod p, solve for x
    let p: u64 = 23;
    let g: u64 = 5;
    let a = rand::thread_rng().gen_range(1..p);
    let b = rand::thread_rng().gen_range(1..p);
    let k_a = mod_pow(g, a, p);
    let k_b = mod_pow(g, b, p);
    let k_b_a = mod_pow(k_b, a, p);
    let k_a_b = mod_pow(k_a, b, p);
    println!("K_a = g^{} = {}, K_b = g^{} = {}, K_b^a = {}, K_a^b = {}", a,k_a,b, k_b, k_b_a, k_a_b);

}
