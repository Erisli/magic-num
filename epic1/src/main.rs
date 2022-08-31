extern crate rand;

use rand::Rng;
fn main() {
    println!("{}",gen_random(255));
    println!("{}",gen_random(231));
}

fn gen_random(max: u8) -> u8 {
    let mut ret:u8 = 0;
    for i in 1..max {
        ret = ret + magic_num::magic_number();
    }
    return ret;
}
