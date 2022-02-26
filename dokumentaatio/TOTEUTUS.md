### Rakenne
---

Ohjelma on jaettu seitsemään kooditiedostoon (luku voi vaihdella, tätä dokumenttia ei välttämättä päivitetä aktiivisesti).

Koodin tulokohta (entry point) sijaitsee `main.rs`-tiedostossa, josta kutsutaan `ui.rs`-tiedostoa, missä käyttöliittymää koskeva koodi sijaitsee. Käyttöliittymässä käytetään Rustyline-kirjastoa mahdollistamaan tuttu komentorivitoiminnallisuus (nuolinäppäimillä tekstin navigoiminen, erilaiset näppäinkomennot jne.).

Käyttöliittymästä käsin kutsutaan `calculator.rs`-tiedostossa sijaitsevan `Calculator`-structin
metodeja. `Calculator` hoitaa muuttujien tallentamisen muistiin sekä muiden funktioiden kutsumisen.

`tokenize.rs`-tiedostossa sijaitsee syötteen parsiva koodi, joka muuttaa syötteen listaksi
tokeneja. Parsimisen jälkeen lista muutetaan infix-muodosta postfix-muotoon
`shunting_yard.rs`-tiedoston funktiossa, mistä on sitten helppo `Calculator`:n laskea arvo
lausekkeelle.


### Aikavaativuus
---

Projektissa on monta funktiota, joiden oma aikavaativuus on O(n). Koska jokaista funktiota kutsutaan kyselyä kohden vain kerran, on koko ohjelman aikavaativuus siis O(k×n) – missä k on kyseisten funktioiden määrä (mikä on vakio). Tästä syystä koko ohjelman aikavaativuus on myös O(n).

### Tilavaativuus
---

Projektin tilavaativuuden arvioinnista en ole ihan niin itsevarma, mutta arvioisin
tilavaativuudenkin olevan O(n). Alkuperäisen syötteen lisäksi luodaan lista tokeneita, jonka
tilavaativuus on noin O(n). Listan lisäksi muistiin ei talleteta mitään muuta, muistinkäyttö
skaalautuu suunnilleen lineaarisesti. 

### Mahdolliset puutteet
---

Isoin puute on, että ohjelmassa ei voi käyttää useamman kuin yhden parametrin funktioita –
`sqrt(4))` toimii kyllä, mutta toisaalta `min(1, 2)` ei.

