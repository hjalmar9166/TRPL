## Kapitel 2
För att importera ett bibliotek från standardbiblioteket skriver man `use std::<biblioteksnamn>;` högst upp i filen. `std::io` är IO-biblioteket.

För att deklarera en ny variabel används `let`. Variabler i Rust är som standard oföränderliga (immutable) men kan göras föränderliga med nyckelordet `mut` på följande vis:
```Rust
let apples = 5;     // Oföränderlig
let mut apples = 5; // Föränderlig
```

För att skapa en ny sträng skriver man `String::new()`.

För att ta in användarindata skriver man `io::stdin()` följt av `.read_line(&must <variable>)` för att lagra indatan i `<variabel>`. För att hantera fel med indata ska man också lägga till `.expect(<felmeddelande>)` efter. Om något går del när programmet ska läsa indata kommar programmet att krasch och felmeddelandet man skriver kommer skrivas ut i konsolen.

Med hjälp av cargo kan man också importera externa bibliotek, så kallade *crates*. I filen Cargo.toml under rubriken `[dependencies]` skriver man biblioteket man vill importera på form `<biblioteksnamn> = <version>`. När man kör `cargo build` compileras de delar av programmet som ändrats sedan senaste kompileringen.

Filen Cargo.lock, som skapas automatiskt första gången man kör `cargo build`.

Genom att importera typen `std::cmp::Ordering` kan värden jämföras med en `match`. `Ordering` tillhandahåller enum:erna `Less`, `Greater` och `Equal` som kan användas för att jämföra värden. De olika fallen i en `match` kallas "arms". För att konvertera en sträng till ett tal kan vi avända funktionen `.parse()` som, precis som `.read_line()`, returnerar en `Result`-typ så att fel kan hanteras med `.expect()`. För att kontrollera vad som händer vid `Ok` och `Err` kan man skriva måsvingeparenteser efter funktionen som returnerar `Result` och sedan explicit skriva vad fallen ska innebära.

En oändlig loop kan skapas med nyckelordet `loop`. Denna kan brytas med `break`. `continue` hoppar till nästa looprunda.

Variabler kan skrivas över så att en sträng t.ex. variabeln `guess` först kan deklaeras som en sträng och sedan skrivas över som en int. Detta kallas *Shadowing*. För att explicit ge en typ till en variabel skriver man `:` efter variabelnamnet och sedan den önskade typen (innan `=`).
