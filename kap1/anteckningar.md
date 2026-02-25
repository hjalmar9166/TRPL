Skriv `cargo new <projektnamn>` för att skapa ett nytt projekt. Detta skapar även ett nytt Git-repo, men inte om projektet redan ligger i ett. För att ändra version control system kan man använda `--vcs`-flaggan när man skapar ett nytt projekt.

Använd `cargo build` för att bygga ett cargo-projekt, `cargo run` för att bygga och köra, och `cargo check` för att kolla om projektet kompilerar utan att faktiskt skapa en körbar fil (går snabbare).

Om man skriver `cargo build --release` görs extra kompilatoroptimeringar som tar längre tid att kompilera men som gör att programmet kör snabbare.
