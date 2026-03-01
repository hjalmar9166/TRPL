## Kapitel 6
### Enum
En enum (enumeration) är en typ som grupperar alternativ tillsammans så att man kan välja exakt en av dem. En frukt-enum skulle kunna deklareras i Rust på följande sätt:
```Rust
enum Fruit {
    Apple,
    Banana,
    Pear,
    Orange,
}
```
Om vi man skapa en instans av `Fruit` som är en banan gör man det sä här: `let fruit = Fruit::Banana;`. Man kan ge enum-alternativen data genom att skriva dess typ inom parenteser efter alternativets namn i deklarationen:
```Rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```
Varje alternativ i en enum är en konstruktor. I exemplet ovan tar konstruktorn in en sträng och returnerar en instans av en `IpAddr`. Olika enum-alternativ kan ha olika typer och i olika antal. Alla möjliga sorters data kan lagras i en enum inklusive structs och andra enums.

Precis som structs kan vi definiera metoder för enums med `impl`.

(Enums liknar algebraiska datatyper i Haskell.)

### Enum:en `Option`
Standardbiblioteket definierar en enum `Option` som antingen är något eller är inget. Detta är som ett mer säkert alternativ till `null`; ett värde Rust inte har. `Option` definieras på följande sätt:
```Rust
enum Option<T> {
    None,
    Some(T),
}
```
En `Option<T>` är inte samma typ som `T` och måste därför konverteras för att kunna användas på samma sätt. Detta säkerställer att det valfria värdet faktiskt är ett värde innan man använder det vilket minskar risken för att programmet beter sig på ett oväntat sätt. Om ett värde inte är en `Option<T>` kan man lugnt veta att det faktiskt är ett värde och inte `null`.

### `match`
Rusts `match` låter en matcha olika mönster för att generera olika ufall. Till skillnad från `if`-staser måste mönstermatchningen i en `match` inte vara b`bool`-värden utan kan vara precis vilken typ om helst. En match för att översätta olika amerikanska mynt till cent kan se ut så här:
```Rust
match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
}
```
Varje arm i `match`:en består av ett mönster, den avgränstande `=>`-operatorn och den villkorliga koden som ska köra för det fallet. Armarna separeras med kommac `,`. Om en arm kräver flera rader kan ett block skapas med måsvingeparenteser efter `=>`. Enums kan också matchas på dess värden så att man kan använda ett värde från enum:en i exekveringsblocket. Värdet sätt då inom parentes efter eum-namnet. På detta sätt kan man få ut ett värde ur en `Option<T>`-enum.

En `match` måste alltid vara uttömmande. Det får inte finnas alternativa möjligheter som en `match` inte täcker. Man kan matcha alla övriga värden (default) till ett generellt namn som man sedan gör något med (sist i `match`-blocket så det triggas efter alla särskilda alternativ testats). Om man inte vill använda värdet kan man använda `_` som matchar alla mönster men som inte binder dem till någon variabel. Om man vill att ingenting ska hända kan man använda den tomma tupeln `()` unit.

### `if let`
Om man bara matchar på ett mönster och inget annat kan man istället använda `if let`-syntax. Exempel:
```Rust
let config_max = Some(3u8);
if let Some(max) = config_max {
    println!("The maximum is configured to be {max}");
}
```
Man kan också lägga till en `else` till en `if let`.

Det finns också `let ... else` som är ett mer kompakt sätt att tilldela ett värde men returnera om värdet inte finns (eller inte uppfyller något annat krav).
