fn main() {
    let mut number = 7;

    if number < 5{     //N.B. Condition of the if must be a bool
        println!("Condition was true!");
    }
    else
    {
        println!("Condition was false!");
    }

    // Multiple conditions and else-ifs
    number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else{
        println!("Number is not divisible by 4, 3 and 2");
    }


    // Using if in a let statement
    let condition = true;
    number = if condition {5} else {6};
    println!("The value of number is {number}");
    // N.B. values that have the potential to be results from each arm of the if must be the same type

    // Loop -> run forever
    loop {
        println!("Again");
        break();// We can use break to exit from loop and add a value to be returned after the break
    }

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop{
        println!("Count = {count}");
        let mut remaining = 10;

        loop{
            println!("Remaining = {remaining}");
            if remaining == 9{
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;

        }

        count += 1;
    }
    println!("End count: {count}");


    // While
    number = 3;
    while number != 0{
        println!("Number = {number}");
        number -= 1;
    }
    println!("LIFTOFF!");

    // For loop
    let a = [10,20,30,40,50];
    for element in a{
        println!("The value is {element}");
    }

    // For loop with range
    for number in (1..4) { // 1 to 4, 4 excluded
        println!("Number = {number}");
    }

    // Range reversed
    for number in (1..4).rev() { // keyword rev
        println!("Number = {number}");
    }



}
