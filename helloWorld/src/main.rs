use core::num;

fn main() {
    let x =10;
    println!("x is {}",x);

    let mut a = 20;
    println!("a = {}",a );

    a = 200;
    println!("a = {}",a );
    println!("Hello, world!");

    let mut num: u64 = 10;
//    num  -= num + 10;
    println!("{}", num );

    let mut numbers = [5,3,5,2,6,7,];
    
    println!("a={}",numbers[0]);

    // tuple
    let mut tupi = (1,2,3,'r');
    println!("{}",tupi.0);


    //functions
    let num= 000;
    println!("{}", myown(num) );
}

fn myown(number: i32) -> i32 {
    return number + number;
}
