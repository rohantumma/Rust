fn main() {
    println!("Hello, world!");
    // strings_here();
    owner_ship();
}

fn loops_here(){
    
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


fn strings_here(){
    // strings and heap storage

    let mut myworld = String::from("Loki");
    println!("{}",myworld);

    myworld.push_str(" is pandas");

    println!("{}", myworld);

}

fn owner_ship (){
    //owner ships

    let owner = String::from("I am here");
    let owner1 = owner;
    // deep copy
    let owner2 = owner1.clone();

    println!("1st{} 2nd{}",owner1,owner2 );
}