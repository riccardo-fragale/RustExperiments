fn main() {
    // println!("Hello, world!");

    //another_function(5);  // functions called by the main coulde be defined both above and below the main

    print_labeled_measurements(5,'h');

    // Statement 
    // let y = 6;
    

    // Expression inside a statement
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is {y}");

    // Calling a function with return value
    let mut result = five();
    println!("The result of the function is {result}");

    result = plus_one(result);
    println!("The result of the plus_one function is {result}");

}

fn another_function(x: i32){ // parameters type MUST be declared in function signatures
    //println!("Another function!");
    println!("The value of x is {x}");
}

fn print_labeled_measurements(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

// Function with return values
fn five() -> i32{
    5
}

// Another examples
fn plus_one(x: i32) -> i32{
    x + 1
}
