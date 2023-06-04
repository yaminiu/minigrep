// my hello world
fn main() {
    /* 
    let mut x  = 10.0; //be default variables are immutable
    println!("x is {}", x);
    x = -3.1415726;
    println!("x is {}", x); // casting integer to float use as
    let a = 10;
    let b = 3.0;
    let c = a as f64 / b;
    println! (" c is {}", c);
    println! (" c is {:.3}", c);
    println! (" c is {:8.3}", c);
    println! (" c is {:08.3}", c);
    println! (" c is {:8.3}\na is {}", c,a); */
    //bitwise operations not and or xor and shift
    let mut value = 0b1111_0101u8;
    println!("value is {}", value);
    println!("value is {:08b}", value);  
    value = !value;
    println!("value is {:08b}", value); 
    println!("bit 6 is {}", value & 0b0100_0000); //check bit is 0 or one 
    value = value | 0b0100_0000; // set bit to 1 but or 
    println!("value bit 6 was set to one and new value is {:08b}", value);
}
