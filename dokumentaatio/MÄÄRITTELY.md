**Opinto-ohjelma**: tietojenkäsittelytieteen kandidaatti

**Dokumentaatiossa käytetty kieli**: suomi. (koodi, muuttujat, kommentit englanniksi)

## Ohjelmointikielet

Projektissa käytetty kieli: Rust

Muita kieliä joita hallitsen (vertaisarvioinnin vaatiman tasoisesti):

- Python
- Java
- Haskell
- C/C++
- Julia
- JavaScript

## Toteutettavat tietorakenteet, algoritmit

[Shunting-yard](https://en.wikipedia.org/wiki/Shunting-yard_algorithm)

## Ratkaistava ongelma

Ohjelmalle voi syöttää [infix](https://en.wikipedia.org/wiki/Infix_notation)-muotoisia matemaattisia lausekkeita (esimerkiksi `1 + 2 * (3 - 4 ^ 5)`), jotka ohjelma kykenee tulkitsemaan ja laskemaan niille vastauksen.

Shunting-yard-algoritmia käytän, koska se lienee tunnetuin tähän soveltuva algoritmi. Shunting-yardin tuottama "[Reverse Polish Notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) (RPN)"-muotoinen lauseke (ylemmässä esimerkissä `1 2 3 4 5 ^ - * +`) on sitten helppo laskea loppuun asti.

Shunting-yardia voisi käyttää myös laskemaan lausekkeen arvo suoraan (ilman, että täytyy muodostaa RPN-lauseke). Haluan tosin yleistää sovelluksen hyväksymään esim. RPN-muotoisia käyttäjän syöttämiä lausekkeita, eikä pelkästään infixiä.

## Aika- ja tilavaativuudet

### Aikavaativuus 

Shunting-yard: 

- O(n)

RPN-lausekkeen laskeminen: 

- O(n)

Koko ohjelman suorituksen tavoiteltu aikavaativuus on siis O(n).

### Tilavaativuus 

Tavoiteltu tilavaativuus on O(n)



