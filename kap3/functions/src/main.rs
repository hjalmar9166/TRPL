fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(10, 'm');
    let fv = five();
    println!("The funktion 'five' returns the value {fv}.");
}

fn another_function(x: i32) {
    println!("The value of x is {x}.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is {value} {unit_label}.");
}

// Eftersom funktionen är en expression har 5 inget semikolon.
fn five() -> i32 {
    5
}
