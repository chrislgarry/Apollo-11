# Prispievanie

🌐
[Azerbajdžanský][AZ],
[bahasa Indonesia][ID],
[Katalánsky][CA],
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
[简体中文][ZH_CN],
[Slovenčina][SK]

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
[SV]:CONTRIBUTING.sv.md  
[TR]:CONTRIBUTING.tr.md  
[UK]:CONTRIBUTING.uk.md  
[VI]:CONTRIBUTING.vi.md  
[ZH_CN]:CONTRIBUTING.zh_cn.md  
[ZH_TW]:CONTRIBUTING.zh_tw.md  
[SK]:CONTRIBUTING.sk.md

Zdrojové kódy v tomto repozitári boli manuálne digitalizované z papierových zdrojových materiálov, a preto sa v nich môžu vyskytovať chyby alebo preklepy. Kód by mal byť upravený tak, aby bol konzistentný so zdrojovým skenom:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Pre jednoduchú navigáciu medzi naskenovanými výtlačkami pre Comanche aj Luminary môžete použiť nasledujúci web: https://28gpc.csb.app/

## Užitočné rozšírenia

GitHub podporuje syntax AGC assembly jazyka. Váš editor ju pravdepodobne nepodporuje, preto tu poskytujeme zoznam rozšírení pre zvýraznenie syntaxe AGC pre nasledujúce editory:

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

**Poznámka:** GitHub a vyššie uvedené rozšírenia automaticky zabezpečia správne formátovanie.

- Odsadenie pomocou tabulátorov  
- Šírka tabu: 8  
- Nenechávajte medzery na konci riadkov

## Čo mám kontrolovať?

Akékoľvek rozdiely medzi skenmi a zdrojovým kódom v tomto repozitári vrátane:

### Komentáre

- Komentáre v prepísanom kóde by mali presne zodpovedať skenom  
  - To môže zahŕňať aj doslovné preklepy alebo pridávanie/odoberanie komentárov

### Zalomenie riadkov

- Zalomenie riadkov *obsahujúcich* `R0000` v stĺpci 1 musí presne zodpovedať skenom.  
- Zalomenie riadkov **bez** `R0000` v stĺpci 1 by malo obsahovať len 1–2 prázdne riadky po sebe.  
  - Ak je ich viac, nadbytočné odstráňte.  
    - Riadky s `R0000` v prvom stĺpci sa do tohto nepočítajú.  
  - Tieto vznikli kvôli nevytištěnému číslu v stĺpci 8 zdrojových dokumentov. Číslo 2 spôsobilo dvojitú medzeru (1 prázdny riadok), číslo 3 trojitú (2 prázdne riadky). Hodnoty 4–8 boli definované, ale nepoužité. Viac v [#159][7]

Príklad:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

By mal byť zmenený na:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0821   LAMPTEST  CS  IMODES33
```

### Medzery

- Medzery medzi znakmi v reťazcoch by mali nasledovať tieto konvencie (viď [#316][10]):  
  - 1 medzera pre nové slovo  
  - 2 medzery pre novú vetu  
  - 3 medzery pre odsadenie

Príklad:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Správne:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Poznámka

Predtým, než otvoríte PR, uistite sa, že vaše zmeny sú konzistentné so skenmi!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master  
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/  
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/  
[6]:https://github.com/wopian/agc-assembly#user-settings  
[7]:https://github.com/chrislgarry/Apollo-11/issues/159  
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/  
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/  
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
