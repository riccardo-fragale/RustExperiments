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


}
