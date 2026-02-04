use core::f64;
pub fn run(){

    //Precision comprison
    let value: f64 = 1.0/3.0;
    let value2: f64 = 1.0/3.0;

    println!("1/3 as f64: {:.20}", value);
    println!("1/3 as f64: {:.20}", value2);

    let large: f64 = 15e10;
    let small: f64 = 2.5e-4;
    let avogadro: f64 = 6.022e23;

    println!("1.5e10 = {}", large);
    println!("2.5e-4: {}", small);
    println!("6.033e23: {}",avogadro);


    let infinity: f64 = f64::INFINITY;
    let neg_infinity:f64 = f64::INFINITY;
    let nan: f64 = f64::NAN;
    let max: f64 = f64::MAX;
    let min: f64 = f64::MIN;
    let min_positive: f64 = f64::MIN_POSITIVE;

    println!("infinity: {}",infinity);
    println!(" negative value: {}",neg_infinity);
    println!("nan value:{}",nan);
    println!("max value: {}",max);
    println!("min value: {}",min);
    println!("min_positive:{}", min_positive);

    let value: f64 = 0.0/0.0;

    println!("is_infinite:{}", f64::INFINITY.is_infinite());
    println!("is_finite: {}", (1.0f64).is_finite());
    println!("is_normal: {}", (1.0f64).is_normal());
    println!("is_nan: {}", value.is_nan());




}