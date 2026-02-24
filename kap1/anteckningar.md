Skriv `cargo new <projektnamn>` för att skapa ett nytt projekt.
`cargo new --vcs=git <projektnamn>` för att inte skapa ett Git repository.

Använd `cargo build` för att bygga ett cargo-projekt, `cargo run` för att bygga och köra, och `cargo check` för att kolla om projektet kompilerar utan att faktiskt skapa en körbar fil (går snabbare).

Om man skriver `cargo build --release` görs extra kompilatoroptimeringar som tar längre tid att kompilera men som gör att programmet kör snabbare.
