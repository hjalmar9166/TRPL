use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Slumpa ett tal mellan 1 och 100 (inklusivt).
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Se det hemliga slumptalet.
    // println!("The secret number is: {secret_number}.");

    // Oändlig loop.
    loop {
        println!("Please input your guess:");

        // Deklarera en ny föränderlig sträng.
        let mut guess = String::new();

        // Lagra användarindata i strängen.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Parse:a strängen till en in. Om strängen inte är ett
        // hoppar programmet bara vidare till nästa looprunda och
        // frågar om ett tal igen.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}.");

        // Kolla om det givna talet är större, mindre eller lika
        // med det sumpade talet (om gissningen är korrekt). Gå
        // ur loopen och avsluta programmet om lika.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
