fn main() {

    let mut num: u8 = magic_num::magic_number();
    // 0-255
    for i in 0..7 {
        if magic_num::magic_number() == 1 {
            num += 2 << i;
        }
    }
    println!("0-255: {num}");

    // 0-231
    num = magic_num::magic_number();
    for i in 0..7 {
        if magic_num::magic_number() == 1 {
            num += 2 << i;
            if num > 231 {
                num -= 2 << i;
                break;
            }
        }
    }

    println!("0-231: {num}");
}
