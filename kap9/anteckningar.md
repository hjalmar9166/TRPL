## Kapitel 9
### `panic!`
Det finns en macro, `panic!` som avslutar programmet när något går olösligt fel. Detta händer automatiskt när man till exempel försöker komma åt ett arrayindex som inte finns, men kan också triggas manuellt med `panic!`.

Genom att köra `cargo run` med `RUST_BACKTRACE=1` kan man få en backtrace av var felet uppstår med alla lager av funktioner som anropats.

### `Resuklt`
För mindre fel som inte kräver att hela programmet kraschar kan man använda enumen `Result`:
```Rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
Här är `T` typen som returneras om allt går som det ska och `E` errortypen man får om det inte för det. Många funktioner från standardbiblioteket returnerar en `Result<T, E>`. Eftersom det finns olika errortyper kan man få programmet att göra olika saker beroende på vilken sorts error som sker.

#### Propagating error
Om man har en funktion där något kan går fel är det ibland bättre att returnera det felet så att man kan bestämma vad man vill göra med det där funktionen anropas istället för att bästämma i förväg i funktionen. Detta görs förstås genom att returnera en `Result<T, E>`. För att göra detta enklare har Rust infört operatorn `?`. Om man placerar ett `?` efter en funktion som returnerar en `Result<T, E>` så kommer detta resultat i sin tur automatiskt returneras av den omslutande funktionen om det är av typen `Err`. Annars fortsätter koden som vanligt och man får själv bestämma vad man gör med det korrkta värdet. `?` konverterar också errorn till samma typ som definierats av funktionens returvärde. `?` kan även användas på returvärden av typen `Option<T>`.

### När ska man nvända vad?
Det är bra att använda `panic!` om koden kan hamna i ett dåligt tillstånd; när något antagande eller någon regel inte gäller. Dessutom bör tillståndet uppfylla något av följande:
* Det är oväntat, till skillnad från sådant som händer ofta som när användaren matar in data.
* Koden som följer kräver att detta dåliga tillstånd inte uppnåts.
* Det finns inget bra sätt att väva in detta tillstånd i typen som används.

### Egna valideringstyper
En möjlighet är att skapa egna valideringstyper. Detta är bra om man har krav som är mer komplicerade och inte enkelt täcks av `Result` eller `Option` och som behövs upprepade gånger i ens kod.
