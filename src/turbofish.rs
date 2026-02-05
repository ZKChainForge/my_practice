
pub fn run(){

    let num: i32 = "42" .parse().unwrap();
    println!("number: {}",num);

    let num = "42" .parse::<i32>().unwrap();
    println!("prised: {}", num);
    let num = "42" .parse::<f64>().unwrap();
    println!("prised: {}", num);
    let num = "42" .parse::<u8>().unwrap();

    println!("prised: {}", num);


}