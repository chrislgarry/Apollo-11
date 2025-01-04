# Contributing

🌐
[Azerbaijani][AZ],
[bahasa Indonesia][ID],
[Català][CA]،
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
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Українська][UK]،
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
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Zdrojové kódy v tomto repozitáři byly manuálně digitalizovány z papírových zdrojových materiálů, mohou se v nich tedy vyskytovat chyby a překlepy. Kód by měl být modifikován tak, aby byl konzistentní se zdrojovým skenem:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Pro snadnou navigaci mezi naskenovanými výtisky pro Comanche i Luminary lze použít následující web: https://28gpc.csb.app/

## Užitečná rozšíření

GitHub podporuje syntaxi AGC assembly jazyka. Váš editor pravděpodobně podporu mít nebude, proto zde poskytujeme seznam rozšíření na zvýraznění syntaxe jazyka AGC pro následující editory:

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

† Podporuje automatické formátování

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

## Formátování

**Poznámka:** GitHub a výše uvedená rozšíření zajistí automatické použití správného formátování.

- Odsazení s použitím tabů
- Šířka tabu je 8
- Nezanechávat bílé znaky (whitespace) na konci řádků

## Co mám kontrolovat?

Jakékoliv rozdíly mezi skeny a zdrojovým kódem v tomto repozitáři, včetně:

### Komentářů

- Komentáře v přepsaném kódu by měly přesně odpovídat skenům
  - Toto může zahrnovat doslovné kopírování překlepů či přidávání/odebírání celých komentářů

### Zalomení řádků

- Zalomení řádků *obsahujících* `R0000` ve sloupci 1 by měly přesně odpovídat skenům.
- Zalomení řádků *__bez__* `R0000` ve sloupci 1 by měly obsahovat jen 1 až 2 prázdné řádky v řadě
  - Pokud obsahují více než dva prázdné řádky, přebytečné odstraňte.
    - Řádky obsahující `R0000` v prvním sloupci se do tohoto nepočítají.
  - Vznikly díky nevytištěnému číslu ve sloupci 8 zdrojových dokumentů. Číslo 2 způsobilo vypsání dvojité mezery (jeden prázdný řádek) a číslo 3 trojité mezery (dva prázdné řádky). Hodnoty 4-8 byly definovány, ale nebyly použity. Více v [#159][7]

Napříkald následující kód:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

By měl být změněn na:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Mezery

- Mezery mezi dvěma znaky v řetězci by měly respektovat tyto konvence (viz diskuze v [#316][10]):
  - Jedna mezera pro nová slova.
  - Dvě mezery pro nové věty.
  - Tři mezery pro odsazení.

Napříkald následující kód:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

By měl být změněn na:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Poznámka

Než otevřete PR, ujistěte se že vaše změny jsou konzistentní se skeny!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
