fn main() {
    println!("Hello, world!");

    let x = 10;
    if x  == 10 {
        println!("true");
    }
    else {
        println!("false");
    }

    // conditioanl assignment

    let mut loki = if x != 10 {100} else {-100};

    println!("loki={}", loki);

    // Loops with break 

    let mut count : i32 = 0;
    loop {
        if count == 10{
            break;
        }
        println!("{}", count);
        count +=1;
    }

    //for loop

    let mut ARR = [1,2,3,4,5];
    for pat in ARR {
        println!("{}", pat);
    }



}
