
fn something(x:i32) -> i32
{
    println!("The value of x is: {x}");
    return x;
}

fn tuples()
{
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    let xx = tup.1; // sets xx to tup[1]
}

fn variables()
{
    let x = 5; // immutable by default
    let mut y = 5; // mutable
    y = 15;
    const Something:i32 = 10;
}

fn arrays()
{
    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // 5 elements of value 3
}

fn control()
{
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let condition = true;
    let number = if condition { 5 } else { 6 };
}

fn looploop()
{
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    // labelled loop
    'outer: loop {
        loop {
            break 'outer
        };
    };

    println!("The result is {result}");
}

fn whileloop()
{
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn forloops()
{
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// moves
fn references1(str: String)
{

}

// references
fn references2(some: &String)
{

}

// references, only one reference can be active at a time
fn references3(some: &mut String)
{

}

fn main() {
    let literal: &str = "hello";
    let xx = String::from("hello");
    let aa = &xx;
    let bb = &xx;


    something(1);

}