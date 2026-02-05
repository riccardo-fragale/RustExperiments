use std::io;

fn main() {
    let x = 5;
    println!("The value of x is {x}");
    let x = x + 1;
    println!("The value of x is {x}");
    {
        let x = x + 2;
        println!("The value of x in the inner scope is {x}");
    
    }
    println!("The value of x is {x}");
    //Constant declaration
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    let xfloat = 2.0; //f64
    let y: f32 = 3.0; //f32

    let f = true;
    let f: bool = false;

    // Tuple type -> they can also be mutable and so their value could be changed
    let tup: (i32,f64,i16) = (500, 6.4, 1);

    // Pattern matching to destructure a tuple
    let (x,y,z) = tup;
    println!("The value of y is {y}");

    // Access a tuple via . notation
    let fivehundred = tup.0;
    let sixpointfour = tup.1;

    // Array -> every element must have the same type. They are allocated on the stack
    let a: [i32; 5] = [1,2,3,4,5];
    // let a = [1,2,3,4,5] is also valid as declaration
    // Arrays can be accessed through [] brackets
    let b = a[3];


    // Invalid array element access
    println!("Please enter an array index");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
                            .trim()
                            .parse()
                            .expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is: {element}!");



}
