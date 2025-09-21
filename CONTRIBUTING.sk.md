



````md
# Prispievanie

🌐
[Azerbaijani][AZ],
[bahasa Indonesia][ID],
[Català][CA],
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Galego][GL],
[Italiano][IT],
[Kurdi][KU],
[Kurdî][KU],
[Lietuvių][LT],
[Mongolia][MN],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Slovenčina][SK],
[Svenska][SV],
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Українська][UK],
[العربية][AR],
[हिन्दी][HI_IN],
[한국어][KO_KR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN]

[AR]:CONTRIBUTING.ar.md
[AZ]:CONTRIBUTING.az.md
[CA]:CONTRIBUTING.ca.md
[CZ]:CONTRIBUTING.cz.md
[DA]:CONTRIBUTING.da.md
[DE]:CONTRIBUTING.de.md
[EN]:../CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[GL]:CONTRIBUTING.gl.md
[GR]:CONTRIBUTING.gr.md
[HI_IN]:CONTRIBUTING.hi_in.md
[ID]:CONTRIBUTING.id.md
[IT]:CONTRIBUTING.it.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[LT]:CONTRIBUTING.lt.md
[MN]:CONTRIBUTING.mn.md
[NL]:CONTRIBUTING.nl.md
[NO]:CONTRIBUTING.no.md
[PL]:CONTRIBUTING.pl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[SK]:CONTRIBUTING.sk.md
[SV]:CONTRIBUTING.sv.md
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Zdrojové kódy v tomto repozitári boli manuálne digitalizované z papierových zdrojových materiálov, preto sa v nich môžu nachádzať chyby a preklepy. Kód by sa mal upravovať tak, aby bol konzistentný s pôvodným skenom:

- [AGC printy pre Comanche][8]
- [AGC printy pre Luminary][9]

Na jednoduchú navigáciu medzi naskenovanými výtlačkami pre Comanche a Luminary je možné použiť nasledujúcu stránku: https://28gpc.csb.app/

## Užitočné rozšírenia

GitHub podporuje syntax jazyka AGC assembly. Váš editor pravdepodobne túto podporu nebude mať, preto tu uvádzame zoznam rozšírení na zvýraznenie syntaxe AGC pre nasledujúce editory:

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

† Podporuje automatické formátovanie

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

## Formátovanie

**Poznámka:** GitHub a vyššie uvedené rozšírenia zabezpečia automatické použitie správneho formátovania.

- Odsadzovanie s použitím tabulátorov
- Šírka tabulátora je 8
- Nezanechávajte biele znaky (whitespace) na konci riadkov

## Čo mám kontrolovať?

Akékoľvek rozdiely medzi skenmi a zdrojovým kódom v tomto repozitári, vrátane:

### Komentárov

- Komentáre v prepísanom kóde by mali presne zodpovedať skenom
  - Toto môže zahŕňať doslovné kopírovanie preklepov či pridávanie/odoberanie celých komentárov

### Zalomenia riadkov

- Zalomenia riadkov *obsahujúcich* `R0000` v stĺpci 1 by mali presne zodpovedať skenom.
- Zalomenia riadkov *__bez__* `R0000` v stĺpci 1 by mali obsahovať len 1 až 2 prázdne riadky za sebou
  - Ak obsahujú viac ako dva prázdne riadky, nadbytočné odstráňte.
    - Riadky obsahujúce `R0000` v prvom stĺpci sa do tohto nepočítajú.
  - Vznikli kvôli nevytlačenému číslu v stĺpci 8 zdrojových dokumentov. Číslo 2 spôsobilo vloženie dvojitej medzery (jeden prázdny riadok) a číslo 3 trojitej medzery (dva prázdne riadky). Hodnoty 4–8 boli definované, ale neboli použité. Viac v [#159][7]

Napríklad nasledujúci kód:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
````

By mal byť zmenený na:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Medzery

* Medzery medzi dvomi znakmi v reťazci by mali rešpektovať tieto konvencie (pozri diskusiu v [#316][10]):

  * Jedna medzera pre nové slová.
  * Dve medzery pre nové vety.
  * Tri medzery pre odsadenie.

Napríklad nasledujúci kód:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

By mal byť zmenený na:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Poznámka

Predtým než otvoríte PR, uistite sa, že vaše zmeny sú konzistentné so skenmi!


