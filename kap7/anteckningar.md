## Kapitel 7
En *crate* är den minsta mängden kod Rust-kompilatorn betraktar vid ett och samma tillfälle. Ett program bestående av en enda källkodsfil är en crate. En crate kan vara antingen *binary crate* eller en *library crate*. En binary crate är ett körbartprogram och måste alltid innehålla en funktion `main`. En library crate å andra sidan har inte en `main`-funktion och kan inte köras i sig utan är en del av ett annat program.

Ett *package* är en bunt crates som tillsammans utgör någon funktionalitet. Ett package innehåller alltid en Cargo.toml-fil som beskriver hur alla crates ska byggas. Ett package kan innehålla flera binary crates men max en library crate, och måste innehålla minst en crate av någon sort.

### Hierarkin av en crate
* Kompilatorn kollar först i crate-roten `src/main.rs` eller `src/lib.rs`.
* I rotfilen kan man deklarera moduler med `mod`.
  * Dessa moduler finns i måsvingeparenteser i samma fil, i `src/<modulnamn>.rs` eller i `src/<modulnamn>/mod.rs`.
* Submoduler deklareras i källfilen för en modul och filerna finns i antingen `src/<huvudmodul>/<submodul>.rs` eller i `src/<huvudmodul>/<submodul>/mod.rs`.
* När en module/submodul finns i en crate kan den nås från vilken annan fil i crate:en som helst med hjälp av `::`.
* En modul är som standard privat men kan göras publik genom att deklareras med `pub mod`. Om man bara vill göra vissa delar publika kan man använda `pub` framför den t.ex. funktionen.
* `use` låter en undvika långa sökvägar och gär att saker i en modul är tillgängliga direkt.

Ex.:

backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs

### Sökvägar
Sökvägar kan antingen vara absoluta och börjar då med nyckelordet `crate` och sedan dess namn, eller relativa och använder då `self` eller `super` eller en identifierare i dess modul. Både absoluta och relativa sökvägar följs av en eller fler identifierare och separeras med `::`.

Saker i en privat submodul kan inte användas av dess förälder men submodulen kan använda dess förälders saker.

Standardpraxis är att ett bibliotek har en binary crate med precis så mycket som krävs i binaren för att starta paketet.

### Publika structs och enums
Om man använder `pub` när man deklarerar en struct blir denna publik men dess fält är fortfarande privata. Gör man å andra sidan en enum publik blir alla dess varianter också publika per automatik.

### `use`
Med nyckelordet `use` kan man föra in sökvägar så att man inte måste skriva hela sökvägen varje gång man vill använda en funtion eller typ från en annan modul. För funktioner är satandard att man för dess föräldermodul in i räckvidden medan för structs, enums mm. är standard att föra in själva saken. Undantaget är om två saker från olika moduler har samma namn, då måste man alltid ha med föräldermodulen.

Nyckelordet `as` låter en byta namn på saker man tar in med `use`. Detta kan också användas för att lösa problemet med likadana namn.

Namn som tas in med `use` är privata för den räckvid de förs in i men kan också göras publika med `pub`.

För att använda externa packet måste man lägga till paketet under dependencies i `Cargo.toml`.

Om man vill använda flera moduler från samma supermodule kan man använda nästlade sökvägar med sakerna man vill ha i måsvingeparenteser och separerade med komma. Ex.:
```Rust
use std::cmp::Ordering;
use std::io;
```
kan instället skrivas
```Rust
use std::{cmp::Ordering, io};
```
Nästlade sökvägar kan användas på alla nivåer av sökvägen. Om man vill ha alla publika saker från en modul kan man använda `* `-operatorn.
