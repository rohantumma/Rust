extern crate core;

use core::slice;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Hello, world!");
    // strings_here();
    // owner_ship();
    // rustModules();
    // rustcrates();
    // clArg();
    // fileRead();
    // fileWrite();
    // structs_here();
    // gneric_struct();
    // treats_here();
    // multiTreats();
    // get_display_call();
    //  match_here();
    // errors_here();
    vector_here();
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

    let number: i32 = rand::random::<i32>();
    // ::<> is a turbofish operater to provide the DataType

    //let number:i32 = rand::random();
    // or can provide DataType at start of variable;

    // it will genrate i32 datatype random int
    println!("{}", number);
}

fn clArg(){
    use std::env;

    for (index, arguments) in env::args().enumerate(){
        println!("{} is {}",index, arguments );
    }

    // access nth only
    println!("2nd element {}", env::args().nth(2).unwrap());
}

fn fileRead(){
    use std::fs;
    let ErrorFound = String::from("Error found");
    println!("file read is -> {}", match fs::read_to_string("testRead.txt") {
        Ok(x) => x,
        Err(_) => ErrorFound,
    });

    // or can do in debugformat printing
    println!("file read is -> {:?}", fs::read_to_string("testRead.txt"))
}

fn fileWrite() {
    use std::fs;
    let text = String::from("fileWriting here only twice");
    fs::write("testWrite.txt",text);
}

#[derive(Debug)]
struct JLR{
    name: String,
    model: String,
    cost: i32
}

// struct methods
impl JLR {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn model(&self) -> &str {
        &self.model
    }
    pub fn cost(&self) -> i32 {
        self.cost
    }
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn set_model(&mut self, model: String) {
        self.model = model;
    }
    pub fn set_cost(&mut self, cost: i32) {
        self.cost = cost;
    }
    pub fn new(name: String, model: String, cost: i32) -> Self {
        Self { name, model, cost }
    }
}

fn structs_here() {
    use std::io;

    //let mut new_price = String::new();
    //io::stdin().read_line(&mut new_price);

    let mut vehical = JLR::new("Range Rover".to_string(),"SV".to_string(),4500000);
    println!("{:?}",vehical);

    // struct update

    let mut vehical2 = JLR {
        name: "Defender".to_string(),
        ..vehical
    };

    println!("{:?}",vehical2);
    // get vehical name
    println!("vehical name is {}",vehical2.name());

    if vehical2.name() == "Defender".to_string(){
        vehical2.set_cost(25000000);
    }
    println!("change cost of {} to {}",vehical2.name(),vehical2.cost());
}

#[derive(Debug)]
struct shape<GenericDataType1,GenericDataType2 > {
    width: GenericDataType1,
    height: GenericDataType2
}

impl<GenericDataType1, GenericDataType2> shape<GenericDataType1, GenericDataType2> {
    pub fn new(width: GenericDataType1, height: GenericDataType2) -> Self {
        Self { width, height }
    }
}

fn gneric_struct() {
    // both width and height of same type
    let rectangle = shape::new(30,40);

    //different type
    let square = shape::new(12.0,12);

    println!("{:?}",rectangle);
    println!("{:?}",square);
}
//////////////////////////////////////////////////
#[derive(PartialEq, PartialOrd)]
struct minnion{
    name:String,
    nature:String,
    no_of_eyes:i32
}

trait Description {
    fn describe(&self) -> String;
}

impl Description for minnion {
    fn describe(&self) -> String {
        format!("Minnion name is: {} \nNature of him: {} \nLooks with {} eys always",self.name,self.nature,self.no_of_eyes)
    }
}

impl minnion {
    pub fn new(name: String, nature: String, no_of_eyes: i32) -> Self {
        Self { name, nature, no_of_eyes }
    }
}

fn treats_here(){
    //ingeneral interface but little differance
    let minion1 = minnion::new("Bob".to_string(),"Kind".to_string(),2);
    println!("{}",minion1.describe());
    let minion2 = minnion::new("Stuart".to_string(),"Egoiesh".to_string(),1);
    println!("here are are with {}", minion1 > minion2)
}
/////////////////////////////////////////////////

use std::fmt;
use std::fs::read_to_string;

fn compare_print<T, U>(x: T, i: U)
    where
        T: fmt::Display + PartialEq + From<U>,
        U: fmt::Display + PartialEq + Copy,
    {
    if x == T::from(i) {
        println!("{} == {}", x, i);
    } else {
        println!("{} != {}", x, i)
    }
}

fn multiTreats() {
    compare_print(1.0, 1);
    compare_print(1.1, 1);
}

fn get_display()-> impl  fmt::Display{
    13
}

fn get_display_call() {
    println!("{}",get_display());
}

fn match_here(){
    println!("0={}",do_match(0))
}

fn do_match(num:i32) -> &'static str {
    let reslut = match num  {
        0 => "zero",
        _ => "Max"// '_' default case // allwas at end
    };
    reslut
}

use std::io;
use std::io::ErrorKind;
use rand::distributions::uniform::SampleBorrow;

fn errors_here(){
    // recover and un recover error -> file not found and index out of error
    // rust do ahve exception
    // result for recover errors
    // panic! for unrevoverable errors with exit code 101
    // panic!("error is here look now");

    // result<T,E>

    let fileOpen = read_to_string("NOFILE.txt");

    let FileContent = match fileOpen {
        Ok(message) => {println!("Got File");}
        Err(error) => match error.kind(){
            io::ErrorKind::NotFound =>{
                println!("File not found, try again");
            }
            _ => {panic!("Other error OWWWWW");}
        }
    };

}

fn vector_here(){
    // vector danamic in size // store in heap
    let mut vecHere:Vec<String> = Vec::new();
    vecHere.push("Loki".to_string());
    vecHere.push("Loki1".to_string());
    vecHere.push("Loki2".to_string());
    vecHere.push("Loki3".to_string());
    vecHere.push("Loki4".to_string());

    println!("{:?}",vecHere);
    println!("vec at index 2 is {:?}",vecHere.get(3));
    hash_map_here();
}

fn hash_map_here(){
    // key has to same dataType for all
    // vale has to same dataType for all

    let mut hash_here = HashMap::new();
    hash_here.insert(1,"loki");
    hash_here.insert(1,"pandas");
    hash_here.insert(2,"Oden");
    println!("{:?}",hash_here);
    println!("{:?}",hash_here.get(&2));
}
