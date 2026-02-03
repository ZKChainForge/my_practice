


pub fn run() {
    let byte: u8 = 255;
    let  small:i16 = -1000;
    let number: i32 = 423;
    let big: i64 = 1_000_000;
    let  huge:i128 =170_111_222_333_444_555_666;
    let size: usize =100;

    print!("integers:{}\n, {}\n, {}\n, {}\n,{}\n, {}\n",byte,small,number, big, huge, size );

    let pi: f64 = 3.14156377678778;
    let e: f32 = 2.71828;

    println!("Floats: pi = {}\n, e = {}\n", pi, e);


    let is_active:bool = true;
    let is_disabled:bool = false;

    println!("Booleans: active = {}, disabled = {}",is_active,is_disabled);

    let letter: char = 'A';
    let name: char = 'y';

    println!("Characters: {}, {}",letter,name);
    


}