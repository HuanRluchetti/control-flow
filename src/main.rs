fn main() {
    println!("----------------------");


    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    println!("----------------------");


    //------------------------------------------------------------------------------\\

    // The error indicates that Rust expected a bool but got an integer. 
    // Unlike languages such as Ruby and JavaScript, Rust will not automatically try 
    // to convert non-Boolean types to a Boolean.

    // if number {
    //     println!("number was three");
    // }

    //------------------------------------------------------------------------------\\

    // I must be explicit and always provide if with a Boolean as its condition. 
    // If we want the if code block to run only when a number is not equal to 0, 
    // for example, we can change the if expression to the following:

    if number != 0 {
        println!("number was something other than zero");
    }

    println!("----------------------");


    //------------------------------------------------------------------------------\\

    // Handling Multiple Conditions with else if

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    println!("----------------------");


    //------------------------------------------------------------------------------\\

    // Using if in a let Statement

    let condition = true;
    let new_number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {new_number}");

    println!("----------------------");

    // his means the values that have the potential to be results from each arm of the if 
    // must be the same type; in Listing 3-2, the results of both the if arm and the else 
    // arm were i32 integers. If the types are mismatched, as in the following example, 
    // we’ll get an error:

    // fn main() {
    //     let condition = true;
    
    //     let error_number = if condition { 5 } else { "six" };
    
    //     println!("The value of number is: {number}");
    // }


    //------------------------------------------------------------------------------\\

    //Repetition with Loops

    let mut counter = 0;

    loop {
        println!("again!");

        counter += 1;

        if counter == 5 {
            break;
        }
    }

    println!("----------------------");


    //------------------------------------------------------------------------------\\

    // Returning Values from Loops

    counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    println!("----------------------");


    // Loop Labels to Disambiguate Between Multiple Loops

    // if I have a multiple loops, I can label him specifying it

    let mut count = 0;

    'counting_up: loop { 
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;  // break labeled loop
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    println!("----------------------");


    //------------------------------------------------------------------------------\\

    // Conditional Loops with while

    let mut while_number = 3;

    while while_number != 0 {
        println!("{while_number}");

        while_number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("----------------------");

    // Looping Through a Collection with for

    let a = [10,20,30,40,50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    println!("----------------------");


    for element in a {
        println!("the value is: {element}");
    }
    println!("----------------------");

    
    for number in 1..4{
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!("----------------------");


    //.rev to reverse the range.
    for number in (1..4).rev() {  
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
    println!("----------------------");



}
