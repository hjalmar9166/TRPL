use std::io;

fn main() {
    println!("Celsius to Fahrenheit:");
    celsius_to_fahrenheit();
    
    println!("\nFibonacci:");
    nth_fibonacci();

    println!("\nTwelve Days of Christmas:");
    twelve_days_of_christmas();
}

fn celsius_to_fahrenheit() {
    println!("Write a temperature in celsius:");
    let mut inpt = String::new();
    io::stdin()
        .read_line(&mut inpt)
        .expect("Failed to read input.");
    let celsius: f64 = inpt.trim().parse().expect("Temperature must be a number.");

    let fahrenheit = 9.0/5.0 * celsius + 32.0;
    println!("{celsius} °C is {fahrenheit} °F.");
}

fn nth_fibonacci() {
    println!("What Fibonacci number do you want?");
    let mut inpt = String::new();
    io::stdin()
        .read_line(&mut inpt)
        .expect("Failed to read input.");
    let f_ind: u32 = inpt.trim().parse().expect("Fibonacci index must be a positive whole number.");

    let fib = fibonacci(f_ind);
    println!("The {f_ind}th Fibonacci number is {fib}");
}

fn fibonacci(ind: u32) -> u64 {
    if ind == 0 {
        return 0
    } else if ind == 1{
        return 1
    }

    let mut f0: u64 = 0;
    let mut f1: u64 = 1;

    for _i in 1..ind {
        let temp = f1;
        f1 = f0 + f1;
        f0 = temp;
    }
    f1
}

fn twelve_days_of_christmas() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "nineth", "tenth", "eleventh", "twelfth"];
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swiming",
        "eight maid a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    for i in 0..12 {
        let d = days[i];
        println!("On the {d} day of christmas my true love gave to me");
        for j in (0..(i+1)).rev() {
            let and = if i > 0 && j == 0 { "and " } else { "" };
            let g = gifts[j];
            println!("{and}{g}");
        }
        println!();
    }
}
