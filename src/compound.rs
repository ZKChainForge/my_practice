
//tuples examples



pub fn run(){
    let point: (i32, i32)= (10,20);
    let person: (&str, i32, bool) = ("ALice", 30, true);
    let data: (i32, f64, char, bool, &str) = (42, 3.14, 'A', true, "hello");
    println!("Point: {:?}",point);
    println!("Person: {:?}", person);
    println!("Data: {:?}", data);

    

    let coordinates: (i32, i32, i32) = (100, 200,300);

    let x:i32 = coordinates.0;
    let y:i32 = coordinates.1;
    let z:i32 = coordinates.2;

    println!("x = {}\n, y = {}\n, z = {}\n", x, y, z);


    let points: (i32, i32) = (50, 100);
    let (x,y) = points;
    println!("x = {}\n, y = {}\n", x, y);

    let (x,_) = points;
    println!("x = {}\n", x);

    let nested: ((i32,i32),(i32,i32)) = ((1, 2), (3, 4));
    let ((a, b),(c, d)) = nested;
    println!("a ,b,c,d :{},{},{},{}",a,b,c,d);

    let mut mut_tup:(i32, i32) = (10,20);
    println!("before: {:?}", mut_tup);
    
    mut_tup.0 = 100;
    mut_tup.1 = 200;
    println!("After: {:?}", mut_tup);


    let numbers:[i32; 5] = [1,2,3,4,5];

    let zeros:[i8; 10] = [0; 10];
    let ones:[i32; 5] = [1; 5];
    let buffer:[u8; 1024] = [0; 1024];

    println!("zeros: {:?}",zeros);
    println!("numbers:{:?}",numbers);
    println!("ones:{:?}", ones);
    println!("buffer:{:?}", buffer);

    let arr = [1,2,3,4,5];
    println!("Array length: {}", arr.len());

    let slice:&[i32] = &arr[1..4];
    let first_three: &[i32] = &arr[..4];
    let last_three: &[i32] = &arr[2..];
    println!("SLICE:{:?}", slice);
    println!("Firstthree:{:?}", first_three);
    println!("last_three:{:?}", last_three);


}