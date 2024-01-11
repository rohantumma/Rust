fn main() {
    println!("Hello, world!");

    let x = 10;
    if x  == 10 {
        println!("true");
    }
    else {
        println!("false");
    }

    //conditioanl assignment

    let mut loki = if (x != 10) {100} else {-100};

    println!("loki={}", loki);
}
