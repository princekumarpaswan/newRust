fn main() {
    println!("hello Prince, I am rust");
    test_fun();
}

fn test_fun() {
    let x: f32 = 10.0;
    let y: u8 = x as u8 - 5;
    let mut iamold: bool = false;
    println!("y: {:?}, iamold: {:?}", y, iamold);
    iamold = true;
    println!("iamold: {:?}", iamold);

    let mystr: &str = "Apple";
    println!("mystr: {}", mystr);

    let name: (&str, &str, u8) = ("Prince", "Kumar",22);
    println!("name: {:?}", name);

    let  array1: [u16; 6] = [ 23, 34, 45, 56, 67, 78];
    println!("array1: {:?}", array1);
    let new_ages: &[u16] = &array1[2..=4];
    println!("new_ages: {:?}", new_ages);
    
}
