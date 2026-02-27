## Kapitel 4
### Ownership
Till skillnad från exempelvis Java där minnet hanteras av en skräpsamlare (garbage collector) och C där minnet måste hanteras manuellt av programmeraren använder Rust ett system för minneshantering de kallar "ownership". Syftet med ownership är att hantera heap-data.

#### Regler för ownership
* Varje värde i Rust har en owner.
* Det kan endast finnas en owner i taget.
* När ownern går ut räckvidden släpps värdet.

### `String`-typen
En strängliteral som `"text"` har ett känt värde och kan därför lagras på stacken. Men om strängen är okänd vid programmets kompilering måste den istället lagras på heapen; då används strängtypen `String`. En strängliteral kan göras om till en trängtyp på fäljande vis: `let s = String::from("text");`. Strängtypen kan som andra typer muteras:
```Rust
let mut s = String::from("hello");

s.push_str(", world!"); // Lägger till delsträng längst bak.

println!("{s}");
```

### Minne och allokering
Som ovan: till skillnad från en strängliteral som har konstant känd storlek och kan läggas på stacken måste en `String` allokeras på heapen. Detta innebär att minnet måste kunna allokeras under körning och att detta minne måste kunna återgå till allokeraren när strängen inte längre används. Det första steget görs när man anropar `String::from`. Det senare är mer komplicerat. I Rust frigörs minnet automatiskt när variabeln som det tillhör inte längre är inom räckvidden. Detta görs via en särskild funktion `drop` som Rust automatiskt anropar vid slutet av en räckvidd.

#### Move
För enkla typer som `i32` kopieras vädet från en variabel till en annan om vi använder tilldelningsoperatorn:
```Rust
let x = 5;
let y = x;
```
`x` och `y` är två separata variabler som båda är bundna till värdet `5`. För strängar är det dock annorlunda.

På stacken finns för en sträng en längd, en kapacitet och en pekare till strängens värde. Strängens värde ligger däremot på heapen. (Längd är strängens längd och kapacitet mängden minne den tlldelats av allokatorn.) Har man en ströng som får sitt värde från en annan sträng görs denna tilldelning referensvis; de två variablerna är separata på stacken (varsin längs/kapacitet/pekare) men pekar på samma minne (det faktiska strängvärdet) på heapen.
```Rust
let x = String::from("text");
let y = x;
```
Detta blir problem eftersom minnet avallokeras när en variabel träder ur räckvidden, vilket kan leda till dubbelt frigörande av minne. Rust betraktar därför `x` i detta fall som att den inte längre är giltig efter att `y` skapats. Rust skulle alltså hindra en från att använda `x` igen efter att `y` skapats. Iställer för en ytlig kopia är detta en förflyttning (move) eftersom `x` görs ogiltigt. Värdet på `x` flyttas till `y`. Detta innebär att Rust *aldrig gör en djup kopia automatiskt*.

#### Räckvidd och tilldelning
På samma sätt avallokeras data automatiskt om vi skriver över ett värde med ett nytt så att ingen variabel längre pekar på det usprungliga värdet:
```String
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
```

#### Clone
Om man däremot *vill* göra en djup kopia av en sträng kan man använda `clone`-metoden:
```Rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
```

#### Ownership och funktioner
Värden flyttas och kopieras på liknande sätt när de skickas som parametrar med funktionsanrop. Detta innebär att en sträng som deklareras och sedan skickas till en funktion blir ogiltig efter funktionsanropet.

Även returvärden kan byta ägare. En sträng som reurneras från en funktion får en ny ägare när den tilldelas till en variabel i den yttre räckvidden.

### Referenser och lån
Istället för att skicka värden kan vi skicka en referens. Därmed förblir originalvariabeln ägare till värdet. En referens görs med `&`-tecknet före variabelnmnet/typen. Eftersom referensen inte äger värdet kommer detta inte att frigöras via `drop` när referensen slutat användas. Detta kallas lån (borrowing) eftersom värdet fortfarande ägs av variabeln där det skapatdes. Som standard är referenser oföränderliga men kan som andra variabler göra föränderliga via nyckelordet `mut`. Både själva deklaratione och funktionsskicket måste göras föränderliga. Endast en föränderlig referens kan göras till ett värde inom samma räckvidd. Detta föhindrar "data reces" som uppstår vid följande händelse:
* två eller fler pekare pekar på samma data på samma gång,
* minst en pekare används för att skriva till datan,
* det finns inge mekanism för att synkronicera åtkomsten.

Man kan inte heller ha både en föränderlig och en oföränderlig referens till samma värde.

En referens räckvidd är till och med sista gången den används, efter vilket nya (o)föränderliga referenser kan göras.

En dinglande referens – en referens för vilken värdet som refereras till lämnat räckvidden – deler till kompileringsfel.

#### Regler för referenser
* Man kan ha natingen *en* föränderlig referens *eller* hur många ofärönderliga referenser som helst.
* Referenser måste alltid vara giltiga.

### Slice-typen
En slice är en sorts referens (har inte ownership) som refererar till intilligande (contiguous) sekvens av data i en samling (collection). En slice för en sträng kan skapas på fäljande vis: `&<strängnamn>[<startindex>..<slutindex>]`. En slice referens har en pekare till startindexet för slice:en och en längd för antalet tecken slice:en innehåller. Om man vill böra på index 0 kan man skippa startindex och vill man sluta på sista elementet kan man slippa slutindex.

En strängliterl är i själva verket en slice som pekar på en specifik plats i binärfilen.

Man inte bara göra slices av strängar utan också arrayer.
