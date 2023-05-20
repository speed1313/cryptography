use std::collections::HashSet;

use rand::Rng;
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn main() {
    // a = r^x mod p, solve for x
    let p: u64 = 6700417;
    // let p: u64 = 2147483647; // very long time to solve x.
    println!("log_2({}) = {}", p, (p as f64).log2());
    let a = rand::thread_rng().gen_range(1..p);
    let r = 5;
    println!("Problem: {}^x = {} (mod {})", r, a, p);

    let mut x = 1;
    for i in 0..p
    {
        x = (x * r) % p;
        if x == a {
            println!("{}^{} = {} (mod {})", r, i, a, p);
            break;
        }
    }

}
