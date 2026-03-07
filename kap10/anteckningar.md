## Kapitel 10
### Generiska datatyper
Generiska datatyper kan användas för att minska kodupprepning i sådana fall där samma kod kan utföras på olika datatyper; till exempel kan man använda generiska datatyper för att sortera både en lista med `char` och en med `i32` med samma funktion. Generiska datatyper brukar markeras med korta, oftast en bokstav, signaturer, vanligen `T`, som sätts mellan krokodilmunnar: `<T>`. För vissa operationer, exempelvis jämförelse, räcker det inte att tillåta vilken typ som helst eftersom operatonen inte är definierad för alla datatyper. Då kan man begränsa den generiska datatypen med en trait.

Funktioner, structs, enums och metoder kan alla skapas med generiska datatyper. För metoder måste den generiska datatypen deklareras efter `impl` för att markera vilken generisk datatyp det handlar om: `impl<T> ...`. Man kan också implementera metoder som bara gäller för secifika typer; detta anges i så fall istället för den generiska datatypen efter `impl`.

Att använda generiska datatyper har ingen påverkan programmets körhastighet eftersom kompiratorn specificerar typerna baserat på hur funktioner mm. aropas/används (kallas *monomorphisation*).

### Traits
En trait definierar funktionalitet hos en viss typ som den kan dela med andra typer. Med traits kan man definiera delade beteenden för flera olika datatyper vilket hjälper med användandet av generiska sådana.

För att skapa en trait används nyckelordet `trait` följt av dess namn och sedan de funktioner som traiten implementerar omslutna avmåsvingeparenteser. Dessa funktioner består endast av funktionshuvudet utan någon kropp. Varje typ som implementerar denna trait måste då också ha en funktion med samma namn och specifikation. En trait implementeras på en struct på fäljande sätt:
```Rust
impl <traitmamn> for <structnamn> {
    ...
}
```

En trait kan också definieras med standardbeteende. Då införs en funktionskropp där detta beteendet definieras. Funktioner med standardbeteende kan även anropa andra funktioner från trait:en, inklusive sådana som inte har något standardbeteende.

#### Traits som parametrar
För att använda traits som parametrar skriver man instället för en fanlig typdeklaration `<parameternamn>: &impl <traitman>`. Då kan funktionen ta emot alla typer som implementerar den trait:en.

Detta är syntaktiskt socker för, och ekvivalenet med, `<T: <traitnamn>>(<parameternamn> &T)`. Detta kallas *trait bound*. Denna syntax är nödvändig om man till exempel har två parametrar med samma trait och vill tvinga dem att också vara av samma typ, vilket inte är möjligt med den mer avskalade syntaxen.

Man kan även kräva flera trait implementationer med `+`.

Om man har många olika traits kan parameterlistan bli lång. Då kan man istället använda `where`-syntax, som skrivs direkt under funktionsdeklarationsraden, ex.:
```Rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```
`some_function` tar in argument av referenser till saker av generisk typ `T` och `U` som alltså kan vara olika, men där `T` måste implementera både `Display` och `Clone` och `U` måste implemetera `Clone` och `Debug`.

Man kan även returnera typer med specifika traits. På samma sätt som med parametrar skriver man då `impl <traitnamn>` där returtypen deklareras. Detta fungerar dock endast om man bara returnerar en sorts typ.

#### Traits som krav på metoder
Man kan implementera en metod omm den implementera en viss trait. Dessutom kan traits implementeras på typer endast om de implementerar en annan trait. Detta används exempelvis av standardbibliotekets `ToString`-trait som implementeras på alla typer som i sin tur implementerar `Display`-trait:en. Detta innebär att metoden `to_string` som definieras i trait:en `ToString` kan anropas på alla typer som implementerar `Display`.

### Lifetimes
En lifetime ser till att referenser är giltiga så länge de behövs. Alla referenser i Rust har en lifetime som utgör den räckvidd i vilken den är giltig; oftast är detta implicit tolkat av kompilatorn precis som typer oftast är implicit tolkade. Men ibland måste man specificera.

Om man till exempel initialiserar en variabel utan värde, sedan inom en inre räckvidd gör variabeln till en referens till en annan variabel som bara är definierad för den räckvidden och sedan försöker använda den ursprungliga variabeln utanför denna räckvidd kommer programmet inte att kunna kompilera:
```Rust
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {r}");
}
```

#### Generic lifetimes i funktioner
Följande kod ska skriva ut den längsta av två strängar:
```Rust
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");
}
```
Eftersom vi inte vill att `longest` ska ta över ägandeskap av variablerna skickar vi dem som referenser i form as tring slices. Om vi skriver `longest` på följande vis kommer pogrammet inte att kompilera:
```Rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() { x } else { y }
}
```
detta eftersom Rust kompilatorn inte kan avgöra om referensen som returneras är till `x` eller `y`. Eftersom vi inte vet de konkreta värdena parametrarna har kan kompilatorn inte veta om `if`- eller `else`-blocket kommer att köra. Livstiden för referensen är också okänd så det går inte att avgöra om en returnerad referens kommer vara giltig. För att fixa detta används generiska livstidsannoteringar.

Livstiden markeras med en apostrof `'` följt av ett vanligen kort namn i gemener. Ofta anvönds `'a` för första livstidsannoteringen osv. Livstidsannoteringen placeras direkt efter `&` följt av ett blanksteg och sedan typsignaturen. Ex.:
```Rust
&i32        // Referens
&'a i32     // Referens med explicit livstid
&'a mut i32 // Föränderlig referens med explicit livstid
```

För att anvönda generiska livstidsannoteringar i en funktion måste de deklareras inom `<>` efter funktionsnamnet precis som generiska datatyper. För exemplet ovan med `longest` vill vi specificera att returreferensen är giltig så länge inparametrarna är det:
```Rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```
När man specificerar livstiden ändrar man inte på den utan talar bara om för kompilatorn att den bör neka anrop med referenser med kortare livstid än vad som angivs. I fallet ovan kommer `'a` ersättas med den kortare av livstiderna för de två parametrarna `longest` tar in, alltså den kortare av `x` och `y`.

#### För structs
Som med generiska datatyper anges den generiska livstiden inom `<>` efter struktnamnet. Om vi ger samma livstide för ett av struct:ens fält som är en referens kommer strukten inte kunna vara giltig längre än det fältet.

#### Utelämning av livstider
I vissa fall behöver man inte specificera livstid eftersom en del mönster finns inbyggda i kompilatorn då de är så vanliga. Dessa mönster kallas *lifetime elision rules*. Kompilatorn avänder sig av följande tre regler för att lista ut livstiden i funktioner:
* Första regeln är att varje parameter får en livstidsannotering.
* Andra regeln är att om det finns exakt en input inputlivstid så får output:en samma livstide.
* Tredje regeln är att om det finns flera input livstider men en är `self` eller `mut self` får output:en samma livstid som `self`.

#### Livstid för metoder
Livstid för metoder deklareras precis som generiska datatyper inom `<>` efter `impl`

#### Statisk livstid
Den statiska livstiden `'static` innebär att en referens kan vara giltig genom hela programmet. Alla strängliteraler har denna livstid. Man bör tänka efter ordentligt om det verkligen är nödvändigt inna man använder denna.
