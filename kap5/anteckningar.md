## Kapitel 5
### Structs
En struct definieras med nyckelordet `struct` följt av dess namn och sedan namn och typ på dess fält (field) inom måsvingeparenteser:
```Rust
struct <Structnamn> {
    <fältnamn>: <typ>,
    .
    .
    .
}
```
För att skapa en instans av en `struct` skriver man dess namn följt av nyckel-värdespar inom måsvingeparenteser. Dessa måste inte komma i samma ordning som de definierades i. För att seadan komma åt fälten används punktoperatorn `.`. För att kunna ändra dessa värden måste hela struct:en också göras föränderlig med `mut`-nyckelordet. En struct kan returneras genom att instansiera den i slutet av en funktion.

Gör man en konstruktorfunktion för en struct kommer man undan med att inte upprepa fältnamnen om fnktionens parametrar är samma som fältnamnen. Ex.:
```Rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

För att skapa en ny struct utifrån en annan struct men endra något fält kan man använda uppdateringssyntax med `..`.Om vi har en struct `s1` av en typ `S`med fälten `f1=4`, `f2=true` och `f3="hej"` och vill skapa en ny struct med samma värden med ändra `f2` kan det göras på följande sätt:
```Rust
let s2 = S {
    f2: false,
    ..f1
}
```
Fält av icke-primitiv datatyper som inte skrivs över kan inte längre användas från den första strukten eftersom dess ägande flyttats till den nya. I detta fall kan vi alltså inte använda `s1.f3` eftersom den flyttats till `s2`.

#### Tupel structs
En tupel struct är som ett mellanting mellan en struct och en tupel. Dess fält har inte namn likt en tupel men själva struct:en har det så tat den kan skiljas från andra tuplar. Man måste fortfarande definiera fältens typ som med en vanlig struct. En RGB-struct skulle kunna defineras så här: `struct Colour(i8, i8, i8);`. Två tupel structs med samma fälttyper är fortfarande olika typer. Som med en vanlig tupel kan man komma åt en tupel structs värden med `.` följt av fältets index eller genom att destructa den: let `Colour(r, g, b) = c;`.

#### Unit-like structs
Man kan även skapa en struct utan fält likt unit-typen (tupel utan värde). En sådan struct deklareras med bara `struct`-nyckelordet och ett namn; inga måsvingeparenteser.

### Derived traits
Om vi vill att en struct ska få någon annan funktionalitet kan vi använda `#[derive(<funktionalitet>)]` innan truct-definitionen. För att till exempel få debugutskrivft kan man skriva `#[derive(Debug)]` och sedan använda `s1:?` (där `s1` är en struct) i en printsats för att få en debuutrskrift av `s1`. Om man instället använde `:#?` skrivs struct:en ut på ett mer lättläst sätt. Ett annat alternativ är att använda `dbg!`-macron som tar ägandeskap över struct:en och skriver ut var i programmet debugutrskriften sker och dess värde.

### Metoder
En metod liknar en funktion och defineras med nyckelordet `fn`, men den definieras inom kontexten för en struct:
```Rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
En metods först parameter är alltid en referens till `self`; structurinstansen i fråga.
