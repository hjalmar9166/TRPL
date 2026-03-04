use std::collections::HashMap;
use std::io;

fn main() {
    median_and_mode();
}

fn median_and_mode() {
    let mut v: Vec<i32> = Vec::new();
    let mut h: HashMap<i32, u32> = HashMap::new();

    println!("Write as many integer numbers as you like. If you want to stop, write 'q'.");
    println!("The median and mode of the values will then be calcualted.");
    loop {
        // Take input from user.
        let mut inpt = String::new();
        io::stdin()
            .read_line(&mut inpt)
            .expect("Failed to read line.");
        let inpt_val = inpt.trim();

        // If "q": stop taking input and continue to calculation.
        if inpt_val == "q" {
            break;
        }

        // Handle inputted value. If not an integer: try again.
        let inpt_val = inpt_val.parse::<i32>();
        match inpt_val {
            Ok(val) => {
                // Add value to the vector to later find the mean.
                v.push(val);
                // Count the number of occurences of each value.
                let count = h.entry(val).or_insert(0);
                *count += 1;
            },
            Err(_) => println!("Please enter an integer number or write 'q' to stop inputting."),
        }
    }

    let num_elements = v.len();

    if num_elements < 1 {
        println!("No values were inputed so there is nothing to calculate.");
    }

    v.sort();
    let median: f64 = if num_elements % 2 == 1 { *&v[num_elements/2-1] as f64 } else { (&v[num_elements/2-1] + &v[num_elements/2]) as f64 / 2.0 };
    let mut mode: i32 = 0; // Dummy value so it is initialised. Will always get a different value
                           // from h.
    let mut num_times: u32 = 0;

    for (key, value) in h {
        if value > num_times {
            num_times = value;
            mode = key;
        }
    }

    println!("The median is {median} and the mode is {mode}");
}
