### Riippuvuudet

Rust (sisältää Cargo:n)
- Systeemin paketinhallintajärjestelmästä (esim. Ubuntulla `apt`)
- Virallisella asennusskriptillä osoitteesta https://www.rust-lang.org/tools/install

(koodikattavuusraportteihin [grcov](https://github.com/mozilla/grcov/) sekä Rustin nightly-versio)

### Käyttäminen

Ohjelma käynnistetään komennolla

`cargo run`

(tai optimoidussa "release"-moodissa komennolla `cargo run --release`)

Ohjelman käynnistäminen käynnistää yksinkertaisen REPL:n (Read Evaluate Print Loop).

Syötä ohjelmalle matemaattisia lausekkeita (esim. `(1 + 2) * 3`) ja ohjelma laskee sille arvon (`(1 + 2) * 3 = 9`).

Luo muuttujia kirjoittamalla muuttujan nimi (esim. "a") ja yhtäsuuruusmerkki "=" ennen lauseketta. Tätä muuttujaa voi käyttää myöhemmissä lausekkeissa normaalin luvun tavoin.

```
>> a = (1 + 2) * 3
 9
>> a + 1
 10
```

Ohjelmasta voi poistua syöttämällä komennon `?quit`.

### Testaaminen

Ohjelmaa voi testata komennolla

`cargo test`

Yksittäisen tiedoston testit voi ajaa komennolla 

`cargo test [tiedoston nimi]`, esim. `cargo test src/main.rs`

### Testikattavuusraportin luominen

Projektin juuressa on tiedosto `create_coverage_raport.sh`, jonka ajamalla voi luoda koodikattavuusraportin `coverage`-kansioon. 

Kansio on jo olemassa ja siinä pitäisi olla ajantasainen kattavuusraportti valmiiksi, tosin.

Kattavuusraportin voi avata selaimeen, esim. Firefox-selaimella komennolla 

`firefox coverage/index.html`

### Koodin dokumentaatio

Rustissa on sisäänrakennettuna hieno rustdoc-työkalu. Sen avulla voi generoida tyylitellyn nettisivun, joka kasaa funktioiden ja niiden parametrien tyypit, sekä ns. doc-kommentit:

`cargo doc --open`

(Antamalla argumentti `--open` cargo avaa dokumentaation selaimeen)
