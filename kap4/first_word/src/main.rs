fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let fw = first_word(&s);

    println!("{hello} {world} fast uppdalade med slices. först ordet är {fw}.");
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Blir problem eftersom originalsträngen kan ändras.
            // Skulle vi vilja ha andra ordet skulle vi istället
            // behöve returnera två värden: index för första och
            // sista tecknet i ordet.
            // return i

            return &s[0..i]
        }
    }

    // s.len()
    &s[..]
}
