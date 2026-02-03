
pub fn run(){

    let tiny: i8 = 100;
    let tiny_negative: i8 = -100;
    let tiny_max: i8 = 127;
    let tiny_min: i8 = -128;

    println!("i8 range:{} to {}", i8::MIN, i8::MAX);
    println!("i8 value: {}, {}, {}, {}", tiny, tiny_negative, tiny_max,tiny_min);


    let small: i16 = 3000;
    let small_negative: i16 = -3000;

    println!("\ni16 range:{} to {}", i16::MIN, i16::MAX);
    println!("i16 values:{}, {}", small,small_negative);
    println!("\ni32 range:{} to {}", i32::MIN, i32::MAX);
    println!("\ni64 range:{} to {}", i64::MIN, i32::MAX);
    print!("\ni128 range:{} to {}", i128::MIN, i128::MAX);
    println!("\nisize range: {} to {}", isize::MIN, isize::MAX);


}