## Kapitel 11
### Att skriva tester
Test ser till att funktioner gör det som avses. En testfunktion gör vanligen följande:
* Förbereder den data eller det tillstånd som testet kräver.
* Kör koden man vill testa.
* Påstår att resultatet är det som förväntas.

Ett test är i sin grund en funktion som annoteras med `test`-attributet. För att göra en funktion till ett test kriver man `#[test]` på raden innan funktionshuvudet. Om man sedan kör `cargo test` bygger Rust en testbinär, kör testerna och rapporterar resultaten. När man skapar ett nytt bibliotek genereras utomatiskt en testfil med en mall för hur man skriver testfunktioner.

I testmodulen kan man ha funktioner som inte är `#[test]`; dessa kan till exempel användas för att förbereda data inför själva testet. För att kontrollera om ett värde är det det förväntas vara kan man använda macron `assert_eq!` som kollar om två värden är lika. Man kan också använda macron `assert!` som evaluerar om ett givet värde är `true`. Det finns även en `assert_ne!` som kollar att två värden *inte* är lika. Det spelar ingen roll vilken ordning man skiver argumenten i `assert_eq!` och `assert_ne!`. Till alla dessa macron kan man också skicka med ett ytterligare argument som är en textsträng som berättar vad som gått fel. Denna sträng kan vara formaterad med `{}`.

Det finns ett attribut `#[should_panic]` som kan testa att en funktion kaschar när den bör göra det. Denna placeras efter `#[test]`. `should_panic` kan klars även om funktionen kraschar av en annan anledning än vad man tänkt sig. Då kan man använda `expect` som kan kräva att felmeddelandet är det rätta. Detta skrivs inom parentes efter `should_panic`, t.ex. `#[should_panic(expected = "less than or equal to 100")]`.

Man kan även använda en `Result<T, E>` i tester.

### Kontroll av hur tester kör
Standard för `cargo test` är att testerna körs parallellt och att output samlas in så att det blir mer lättöverskådligt. Men man kan ge kommanoargument som gör att testerna körs på annat sätt. Om testerna beror på varandra och man inte vill köra dem parallellt på olika trådar kan man använda argumentet `--test-threads` fölt av likhetstecken och hur många trådar man vill köra. Om man exempelvis bara vill ha en kör man testerna med `cargo test -- --test-threads=1`.

Om man vill att output från funktioner ska skrivas ut kan man använda argumentet `--show-output`.

Om man bara vill köra ett test kan man skicka med testets namn som parameter. Vill man köra ett urval av tester kan man också specificera en del av ett namn, så körs alla tester vars namn har den strängen som en del. På så vis kan man också till exempel köra alla tester som hör till en viss modul eftersom modulens namn också är en del av testets namn.

Om man vill skippa specifika tester kan man anvönda attributet `ignore` som också skrivs under `test` över funktionshuvudet. För att köra endast de tester med `ignore`-attributet kan man ge `cargo test` argumentet `--ignored` och för att köra alla tester kan man ge argumentet `--include-ignored`.

### Organisering av tester
Det finns två sorters tester: enhetstester (unit tests) och integreringstester (integration tests). enhetstester testar små delar av koder, en modul i taget och kan testa privata metoder. Ett integrationstest å andra sidan testar hela biblioteket och anvönder det precis som vilken annan kod som helst, med samma åtkomst.

#### Enhetstester
Enhetstester testar enskilda moduler. Dessa tester läggs i `src` vanligen i en modul `test` med annotering `cfg(test)`. Detta gör att koden endast kompileras när man kör `cargo test` och inte när man kör `cargo build`.

Rust tillåter att test kör privata funktioner/metoder.

#### Integreringstester
Integreringstester testar ett bibliotek i helhet utifrån eftersom delar som fungerar bra för sig kan strula när de kombineras. För att göra ett integreringstest måste man först ha en `tests` katalog parallellt med `src`. Varje fil i denna katalog blir en egen crate. Dessa körs allihop vid `cargo test` men för att endast köra från en specifikt test-crate kan man använda argumentet `--test` följt av crate:ens namn.

För att ha submouler som t.ex. kn nås av flera tester måste man lägga filen i en katalog med modulens namn och kalla själva filen för `mod.rs`. Filer i underkataloger till `test` kompileras inte som egna crates.
