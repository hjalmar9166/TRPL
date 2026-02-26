## Kapitel 2
### Oföränderlighet och `mut`
Som redan etablerat är variabler oföränderliga (immutable) som standard. Man kan skapa en föränderlig variabel genom att använda nyckelordet `mut`.

### Konstanter
Konstanter deklareras med nyckelordet `const` istället för `let` och dess datatyp måste anges explicit. Konstanter kan inte göras föränderliga med `mut`. Konstanter kan deklareras inom alla räckvidder, även globalt. Konstanter måste skapas med ett värde som är ett konstant uttryck och inte ett som beräknas under körning. Konventionenn för konstantnamn i Rust är versaler med understreck mellan ord.

### Shadowing
Man kan deklarera en ny variabel med samma namn som en tidigare variabel. Detta kallas shadowing. Shadowing görs genom att använda samma variabelnamn som tidigare och en ny deklaration med `let`. Om shadowing sker inom en inre räckvidd återgår variabeln till föregående värde när programmet går ur den inre räckvidden.

Till skillnad från `mut` låter shadowing en edast göra ändringar på variabeln precis när man använder `let`, efter vilket variabeln återgår till att vara oföränderlig. En annan skillnad är att man kan byta variabelns typ när man använder shadowing eftersom det skapar en ny variabel vilket inte går med `mut` eftersom man inte kan mutera datatypen. Detta kan vara praktiskt i vissa fall.

### Datatyper
Rust är statiskt typat och måste alltså veta alla variablers typer vid kompilering. Kompilatorn kan oftast lista ut vilken typ en variabel ska ha utan att man anger det uttryckligen. Men i vissa fall måste man specificera som när man använder `parse`.

#### Skalära typer
Skalära typer (scalar types) representerar enskilda värden. Rusts fyra huvudsakliga skalära typer är integers, flyttal, Boole-värden och karaktärer.

**Integer**
En integer specificeras med ett `i` för signed och `u` för unsigned följt av antalet bitar från 8 till 128 i tvåpotenser. Dessutom finns `isize` och `usize` som beror på datorns architektur. Integers kan skrivas i olika talbaser på följande vis: `0x` för HEX, `0o` för OCT, `0b` för BIN, och `b'A'` för byte med `u8`; och kan separeas för läslighet med ett understräck (ex. `10_000`).

Det finns ett antal sätt att hantera integer overflow på. (Inte så tydligt förklarat men jag antar att det kommer mer om det sen.)

**Flyttal**
Flyttal kan vara natingen `f32` eller `f64` med det sistnämnda som standard. Alla flyttal är signed.

**Boolean**
Nyckelorden för `bool` skrivs med små bokstäver i Rust: `true` och `false`. Booleans är en byte stora.

**Karaktärer**
En `char` är precis som man förväntar sig en enda karaktär och specificeras med enkla cituationstecken 'A'. En `char` i Rust är fyra byte och reprecenterar Unicodevärden.

#### Sammansatta typer
**Tupel**
En tupel har en fix längd och kan innehålla data av olika typer. Tupler definieras med parenteser runt och komman mellan de olika värdena. Typdefinitionen görs på samma sätt. Värden kan fås ur en tupel via pattern matching. Ett tupelvärde kan också nås direkt via `.`-operatorn följt av indexet av värdet. En tom tupel kallas en *unit* och representerar ett tomt värde. En tom tupel returneras alltid om inget annat returvärde ges till ett uttryck.

**Array**
Alla element i en array måste ha samma typ och arrayer har fix längd. Arrayer sparas på stacken. En arrays typ skrivs med hakparenteser runt dess typ och längd separerat med ett semikolon (ex. `let a: [i32; 5] = ...`). Man kan skapa en array där alla element är samma genom att skriva värdet följt av ett semikolon och sedan längden, omslutet av hakparenteser (ex. `let a = [1, 5]`). Element i en array kan kommas åt via `[]`-operatorn.

### Funktioner
