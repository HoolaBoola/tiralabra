### Yksikkötestaus

Yksikkötestaukseen on käytetty Rustin sisäänrakennettuja testimoduuleja (kooditiedoston lopussa, merkitty `#[cfg(test)]`). Testit voi ajaa komennolla `cargo test`.

Testeistä voi generoida kattavuusraportin projektin juuresta löytyvällä `create_coverage_report.sh`-tiedostolla (vaatii Rustin nightly-version ja grcov-työkalun).

Valmiiksi generoitu raportti löytyy myös projektin juuresta, `coverage`-kansiosta. Raportin saa avattua selaimeen komennolla `xdg-open coverage/index.html` (`xdg-open`:n pitäisi avata se selaimessa, mutta voi myös korvata halutun selaimen nimellä, esim. `firefox` tai `chromium`).

Testikattavuusraportin kiteyttävä "badge" löytyy myös kansiosta – se näkyy README.md-tiedostossa sekä alla:

![Coverage badge](../coverage/badges/flat.svg)

(huom.! `main.rs`- ja `ui.rs`-tiedostoille ei ole yksikkötestejä, mikä laskee badgen prosenttilukua hieman)

---

Yksikkötestien kirjoittaminen tähän projektiin on melko suoraviivaista – syötteet ja niiden perusteella haluttavat tulokset ovat selkeitä ja melko pieniä. Esimerkiksi syötteellä `(2+3)^4 * 3 - 1 + 4` pitää tulostaa 1870 ollakseen oikein.

Suorituskykytestaus ei tässä projektissa ole kovin mielekästä – suuretkin laskutoimitukset suoriutuvat sekunnin murto-osassa lineaarisen aikakompleksisuuden takia.

---

### Tyylin testaus 

Rustissa on oletuksena checkstyle-tyylinen, hyvät defaultit omaava työkalu Clippy (aja komentorivillä käyttäen `cargo clippy`). Clippyn tulostus saattaa olla esimerkiksi tällainen:

```
   |
78 |             if var_list.len() == 0 {
   |                ^^^^^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `var_list.is_empty()`
```

Tarkoitus on ottaa käyttöön Github Actioneissa tämän automatisointi.
