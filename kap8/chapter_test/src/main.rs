use std::collections::HashMap;
use std::io;

fn main() {
    median_and_mode();
    pig_latin();
    company();
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
    let median: f64 = if num_elements % 2 == 1 { *&v[num_elements/2] as f64 } else { (&v[num_elements/2-1] + &v[num_elements/2]) as f64 / 2.0 };
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

fn pig_latin() {
    println!("Write a sentence and it will be translated into pig latin:");
    let mut inpt = String::new();
    io::stdin()
        .read_line(&mut inpt)
        .expect("Failed to read line.");

    // Split the words on space.
    let words = inpt.split(' ');
    let mut new_words = String::new();
    // Go through every word and translate to pig latin.
    for w in words {
        new_words.push_str(translate_to_pig_latin(w).trim());
        new_words.push(' ');
    }

    println!("{}", new_words);
}

fn translate_to_pig_latin(w: &str) -> String {
    let vowels = ['i', 'e', 'u', 'o', 'a'];
    let mut chars = w.chars();
    let fst_letter = chars.next();
    // Check for valid string.
    match fst_letter {
        Some(c) => {
            // If bowel: add "hay".
            if vowels.contains(&c) {
                return format!("{c}{}hay", chars.as_str())
            }

            // Otherwise: move first letter to the back and add "ay".
            return format!("{}{c}ay", chars.as_str())
        },
        None => return String::from(w)
    }
}

fn company() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();

    retrieve_employees(&mut employees);
    print_employees(&employees);
}

fn retrieve_employees(employees: &mut HashMap<String, Vec<String>>) {
    println!("Add people to departments 'add name to department'.");
    loop {
        let mut inpt = String::new();
        io::stdin()
            .read_line(&mut inpt)
            .expect("Couldn't read line.");
        let inpt_val = inpt.trim();

        // Stop inputting.
        if inpt_val == "q" {
            break;
        }

        // Split the input into individual words.
        let words: Vec<&str> = inpt_val.split(' ').collect();

        // If the input is not the correct format: redo.
        if words[0] != "add" || words[2] != "to" || words.len() != 4 {
            println!("Incorrect input format.");
            continue;
        }
        
        let department: &mut Vec<String> = employees.entry(String::from(words[3])).or_insert(Vec::new());
        department.push(String::from(words[1]));
    }
}

fn print_employees(employees: &HashMap<String, Vec<String>>) {
    if employees.is_empty() {
        println!("There are no employees.");
        return
    }

    loop {
        println!("What department do you want to see?");
        let mut departments: Vec<&String> = employees.keys().collect();
        departments.sort();

        // Print department names.
        for i in 0..departments.len() {
            println!("{i}. {}", &departments[i]);
        }
        println!("{}. all", departments.len());

        let mut inpt = String::new();
        io::stdin()
            .read_line(&mut inpt)
            .expect("Couldn't read line.");
        let inpt_val = inpt.trim();

        if inpt_val == "q" {
            break;
        }

        let opt = inpt_val.parse::<usize>();

        match opt {
            Ok(n) => {
                if n > departments.len() {
                    println!("Incorrect input.");
                    continue;
                }

                print_departments(n, employees, &departments);
            },
            Err(_) => {
                println!("Incorrect input.");
                continue;
            },
        }
    }
}

fn print_departments(n: usize, employees: &HashMap<String, Vec<String>>, departments: &Vec<&String>) {
    if n == departments.len() {
        // Print all departments.
        for &dep in departments {
            println!("{}\n----------", dep);
            let emp = employees.get(dep).expect("Something went wrong.");
            for i in 0..emp.len() {
                println!("{}", &emp[i]);
            }
            println!();
        }
    }
    else {
        // Print selected department.
        let emp = employees.get(departments[n]).expect("Something went wrong.");
        println!("{}\n----------", departments[n]);
        for i in 0..emp.len() {
            println!("{}", &emp[i]);
        }
        println!();
    }
}
