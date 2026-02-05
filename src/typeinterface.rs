
pub fn run(){
    let integer: i32 = 42;
    let float : f32 = 3.14;
    let boolean = true;
    let character = 'A';
    let string = "hello";


    fn type_of<T>(_: &T) -> & 'static str{
        std::any::type_name::<T>()
    }

    println!("42 {}", type_of(&integer));
    println!("3.14 {}", type_of(&float));
    println!("'A' {}", type_of(&character));
    println!("true {}", type_of(&boolean));
    println!("hello\n {}", type_of(&string));

}