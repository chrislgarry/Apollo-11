# Prisidėti

🎌
[Čeština][CZ],
[Deutsch][DE],
**Lietuvių**,
[Español][ES],
[Français][FR],
[Kurdi][KU],
[Nederlands][NL],
[Português][PT_BR],
[Türkçe][TR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CZ]:CONTRIBUTING.cz.md
[DE]:CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[JA]:CONTRIBUTING.ja.md
[LT]:CONTRIBUTING.lt.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[NL]:CONTRIBUTING.nl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Šios saugyklos kodas buvo skaitmenizuotas iš popierinių spausdinių, todėl galima tikėtis klaidų ir kiti neatitikimų. Kodas turi būti pakeistas, kad taptų labiau nuoseklus su skenuotais spausdiniais:

- [AGC spausdiniai Comache][8]
- [AGC spausdiniai Luminary][9]

## Naudingi plėtiniai

GitHub turi integruotą AGC assemblerio kalbos sintakės palaikymą. Dėja, jūsų kodo redaktorius nepalaikys, tačiau yra AGC kalbos plėtinių, kurie suteikia sintakės paryškinimą šiems redaktoriams:

- [Atom][Atom]†
- [CodeBlocks][CodeBlocks]
- [Eclipse][Eclipse]
- [Kate][Kate]
- [ProgrammersNotepad][ProgrammersNotepad]
- [Sublime Text 3][Sublime Text]†
- [TextPad][TextPad]
- [Vim][Vim]
- [Visual Studio Code][VisualStudioCode]†
- [jEdit][jEdit]

† Palaiko automatinį formatavimą

[Atom]:https://github.com/Alhadis/language-agc
[CodeBlocks]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/CodeBlocks
[Eclipse]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Eclipse
[Kate]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Kate
[ProgrammersNotepad]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/ProgrammersNotepad
[Sublime Text]:https://github.com/jimlawton/AGC-Assembly
[TextPad]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/TextPad
[Vim]:https://github.com/wsdjeg/vim-assembly
[VisualStudioCode]:https://github.com/wopian/agc-assembly
[jEdit]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/jEdit

## Formatavimas

**Pastaba:** GitHub ir plėtiniai nurodyti viršuje, užtikrins, kad jūs naudojate teisingą formatavimą.

- Naudokite skirtuko įtrauką
- Naudokite skirtuko plotį 8
- Ištrinkite galinius tarpus

## Ką man reikia patikrinti?

Betkokius neatitikimus tarp skenuočių ir šaltinio kodo šioje saugykloje, įskaitant:

### Komentarai

- Komentarai nurašytame kode turi atitikti skenuočių komentarus tiksliai
  - Gali reikėti sąmoningai pridėti klaidas ar netgi pašalinti/pridėti visą komentarą.

### Eilučių lūžiai

- Linijų lūžiai su *pločiu* `R0000` pirmame stulpelyje turi tiksliai atitikti skenuotes.
- Linijų lūžiai su *pločiu**__out__* `R0000` pirmame stulpelyje gali turėti tik vieną arba dvi tuščias eilutes iš eilės.
  - Jeigu yra daugiau negu dvi tuščios linijos, reikia ištrinti papildomas eilutes.
    - Linijos su `R0000` pirmame stulpelyje nesiskaičiuoja.
  - Šaltinių nuotraukose, šitie buvo sukurti nespausdintu skaitmeniu aštuntame stulpelyje. A 2 - ten buvo dvigubas tarpas (viena tuščia eilutė) ir a 3 buvo trigubas tarpas   (dviguba tuščia eilutė). Reikšmės nuo 4-8 buvo apibrėžtos, bet niekada nenaudotos. Daugiau apie tai [#159][7]

Pavyzdžiui, šis tekstas:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Turėtų patapti:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Tarpai

- Tarpai tarp dviejų simbolių eilutėje turi atitikti šią konvenciją (daugiau diskusijoje [#316][10]):
  - Vienas tarpas naujiems žodžiams.
  - Dvigubas tarpas naujiems sakiniams.
  - Trigubas tarpas įtraukoms.

Pavyzdžiui, šis tekstas:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Turėtų patapti:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Pastabos

Prieš kuriant PR, prašome įsitikinti, kad jūsų pakeitimai atitinka skenuotes!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
