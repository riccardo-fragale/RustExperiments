#[derive(Debug)] // Debug trait
struct Rectangle{
    width: u32,
    height: u32
}

fn main() {

    //let width1 = 30;
    //let width2 = 50;

    //let rect1 = (30,50);
    let scale = 2;

    /*
    Another way to print out a value using the Debug format is to use the dbg! macro, 
    which takes ownership of an expression (as opposed to println!, which takes a reference), 
    prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, 
    and returns ownership of the value.
    Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout). 
    */
    let rect = Rectangle{
        width: dbg!{30 * scale},
        height: 50
    };

    // Debug
    println!("Rectangle is {rect:?}");


    println!("The are of the rectangle is {} square pixels.",
        area_struct(&rect));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32,u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
