## Kapitel 8
Collections från `std`-paketet.

### Vektorer
För att skapa en tom ny vektor använder man funktionen `Vec::new`: `let v: Vec<i32> = Vec::new();`. Eftersom en vektor kan skapas för att innehålla alla element av vilken datatyp som helst måste man specificeras vilken typ man vill ha. För att skapa en vector med initialvärden kan man använda macron `vec!`:
`let v = vec![1, 2, 3];`. Då kan kompilatorn dra egna slutsatser om vilken typ vektorn ska ha.

För att lägga till element i en vektor använder man metoden `push`. För att detta ska fungera måste vektorn deklarerar som föränderlig med `mut`. Det finns två sätt att läsa värden lagrade i en vektor: via indexering eller med metoden `get`. Indexering `&v[i]` ger en `&i32` och `get` `v.get(i)` en `Option<&i32)` (utifrån exemålet ovan). För att kunna ändra värden i en föränderlig vektor måste man avreferera den med `*`-operatorn.

Alla element i en vektor måste vara av samma typ men med enums kan man ha flera typer.

Vectorer har även metoden `pop` för att ta bort och returnera vektorns sista element.

Om man dropar en vektor dropas alla dess element.

### Strängar
Det finns två sorters strängar: `str` eller oftare använd som `&str` som är en del av Rusts grundtyper; och `String` som är den typ i standardbiblioteket. Båda är UTF-8-format.

Precis som `Vec` har `String` en `new`-metod som används för att skapa en ny sträng. För att skapa en sträng av någon redan existerande data använder man `to_string`-metoden som finns för alla typer som implementerar `Distplay`. `to_string` fungerar även på literalsträngar men man kan också använda `String::from` för att omvandla en literal till en sträng.

För att konkatenera strängar kan man använda `+`-operatorn eller `format!`-macron. För att lägga till en sträng i slutet på en annan finns `push_str` som tar en string slice så literaler fungerar här. Metoden `push` tar in ett enda tecken och lägger till längst bak i en sträng.

Om man använder `+` för att konkatenera strängar så invalideras den första strängen medan denandra måste skickas som referens (och alltså är en string slice): `let s3 = s1 + &s2;`.

En sträng i Rust kan inte indexeras eftersom Unicodekaraktärer kan ha olika längd.

#### Bytes, skalärer och grafemkluster
Det finns tre sätt att se på en sträng i Rust
* Byte – varje enskilt byte-värde som sparas i strängen. Dessa är bara tal i minnet.
* Skalär – Unicodekaraktärer motsvarande Rusts `char`. Dessa innehålelr också diakritiska tecken.
* Grafemkluster – vad som motsvarar faktiska "bokstäver".

För att iterera över bitarna i en sträng kan man använda metoden `bytes` och för att iterera över Unicodekaraktärerna metoden `chars`.

### Hashtabell
En hashtabell `HashMap<K, V>` sparar en mappning av en nyckel av typ `K` och ett värde av typ `V` med hjälp av en hashfunktion. Även en hashtabell skapas med `new`-metoden och nya nyckel-värde-par läggs in med metoden `insert`. Till skilnad från de två ovannämnda måste `HashMap` hämtas in från `std::collections` via `use`.

För att sedan hämta ut ett värde använda metoden `get`. Detta returnerar en `Option<&V>`, som kan göras om till en `Option<V>` med metoden `copied`. För att få ut värdet och hantera värden som inte finns kan man använda `unwrap_or` med default-värde som argument.

#### Updatering
Anropar man `insert` med en nyckel som redan finns skrivs det existerande värdet över (givet att hashtabellen är föränderlig förstås). För att bara lägga till ett värde om nyckeln inte redan finns kan man använda metoden `entry` med nyckeln som parameter följt av `or_insert` med värdet man vill lägga in om inget redan finns. En tredje möjlighet är att man vill uppdatera ett värde baserat på vad som är lagrat på den nyckeln. Då kan man avreferera värdet med `*`.
