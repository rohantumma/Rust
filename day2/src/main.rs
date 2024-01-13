fn main() {
    println!("Hello, world!");
    // strings_here();
    // owner_ship();
    // rustModules();
    rustcrates();
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

    // Brrowing RF

    let nameToBe = String::from("I am Loki");
    let nameWantToBe = &nameToBe;

    println!("{} is RF of {}",nameToBe,nameWantToBe );

    //mutable RF
    let mut refZero = String::from("mut REF");
    let ref1 = &mut refZero;

    // slice

    let message = String::from("going to break");
   // [start Part of string Length.. with end part]
   // or provide end part only
   // or start part only
    let SlicedMessageIM = &message[9..14];
    println!("{}", SlicedMessageIM);
}

fn rustModules () {
    // std
    // use std::

    // there are some prelude which import most coommon std's
    // like string, cloan, etc
    
    // i/o mudule
     use std::io;

     let mut buffer =String::new();

     io::stdin().read_line(&mut buffer);
    // println!("typed {}", buffer);

     //parse the string
    let number:i32 = buffer.trim().parse().unwrap();
    // trim to remove \n from end of string wich alws prenst
    // parse() to parse and unwrap() if user tried to pass other datatype 
    // apart form string int values
    println!("increment with 1 {}", number + 1 );

}

fn rustcrates(){
    //external libs
    //see https://crates.io/
    // add dependancy to toml file
    use rand;

    let number = rand::random::<i32>();
    // ::<> is a turbofish operater to provide the DataType

    //let number:i32 = rand::random();
    // or can provide DataType at start of variable;

    // it will genrate i32 datatype random int
    println!("{}", number);
}