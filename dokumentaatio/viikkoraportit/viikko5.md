Käytetyt tunnit: ~10

### Mitä tein tällä viikolla?

Ohjelmassa pystyy nyt syöttämään negatiivisia numeroita (aiemmin pystyi *vähentämään positiivisia*
        numeroita, esim. `0 - 1 == -1`, muttei voinut syöttää niitä esim. lausekkeen ensimmäisenä
        arvona).

Löysin muutaman bugin ja selvitin ne. Samalla tulin ottaneeksi projektin ensimmäisen kirjaston
käyttöön, kun otin [Rustylinen](https://github.com/kkawakam/rustyline) hoitamaan tuttuja komentoja
(esim. CTRL-W poistaa sanan, CTRL-vasen hyppää sanan vasemmalle, CTRL-A hyppää lausekkeen alkuun).
Rustylinen käyttöönotto ratkaisi myös ärsyttävän bugin, jonka takia ohjelmalle ei voinut kopioida
yli 4096 merkin lausekkeita.

Päivitin ja lisäsin osuvia virheviestejä paikoitellen.

Kirjoitin lisää testejä ja dokumenttikommentteja.

### Mitä opin?

Suurin osa Linux-terminaaleista rajoittaa oletuksena pasteamisen 4096 merkkiin. 

### Mitä seuraavaksi?

Funktioiden parsiminen ja niillä laskeminen (edelleen listalla).

Esimerkki:

```
> sqrt 2 
1.4142135623730951
> sin(1) 
0.8414709848078965
```
