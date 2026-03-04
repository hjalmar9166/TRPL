mod front_of_house; // Deklarerar en modul.

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Apertizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Anrop till funktion från förälder.
    }

    fn cook_order() {}
}

fn deliver_order() {}

pub fn eat_at_restaurant() {
    // Absolut sökväg:
    crate::front_of_house::hosting::add_to_waitlist();

    // Relativ sökväg:
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast, please.", meal.toast);

    let order1 = back_of_house::Apertizer::Soup;
    let order2 = back_of_house::Apertizer::Salad;
}
