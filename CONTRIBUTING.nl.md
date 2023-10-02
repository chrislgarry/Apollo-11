# Bijdragen

🎌
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
**Nederlands**,
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Türkçe][TR],
[Ukrainian][UA]،
[Ελληνικά][GR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CZ]:CONTRIBUTING.cz.md
[DA]:CONTRIBUTING.da.md
[DE]:CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[GR]:CONTRIBUTING.gr.md
[IT]:CONTRIBUTING.it.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[LT]:CONTRIBUTING.lt.md
[NL]:CONTRIBUTING.nl.md
[NO]:CONTRIBUTING.no.md
[PL]:CONTRIBUTING.pl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[UA]:CONTRIBUTING.ua.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

De broncode in deze repository is handmatig gedigitalizeerd van papieren stukken. Typfouten en andere discrepanties kunnen per ongeluk zijn toegevoegd. De code zal worden aangepast om gelijk te worden gemaakt aan de gescande papieren:

- [AGC prints voor Comanche][8]
- [AGC prints voor Luminary][9]

## Handige extensies

GitHub heeft syntax ondersteuning voor de AGC assembly taal ingebouwd. Helaas heeft jouw editor dit niet, gelukkig zijn er AGC extensies die de volgende editors kunnen voorzien van syntax ondersteuning:

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

† Ondersteunt automatische opmaak

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

## Opmaak

**Let op:** GitHub en de extensies in de lijst hierboven zorgen ervoor dat je automatisch de correcte opmaak gebruikt.

- Gebruik tab indentatie
- Gebruik een tab width van 8
- Haal trailing whitespace weg

## Wat controleer ik?

Elk verschil tussen de scans en de broncode in deze repository inclusief:

### Commentaren

- Commentaren in de overgeschreven code moeten exact hetzelfde zijn als in de scans
  - Soms betekent dit dat je expres een typfout moet overnemen of een commentaar volledig moet weghalen.

### Regeleindes

- Regeleinden *met* `R0000` in kolom 1 moeten exact overeen komen met de scans.
- Regeleinden *zonder* `R0000` in kolom 1 moeten slechts 1 of 2 lege regels in een rij bevatten.
  - Als er meer dan twee lege regels zijn, haal de extra witregels dan weg.
    - Regels met `R0000` in kolom 1 tellen hierbij niet mee.
  - In de bron scans zijn deze aangemaakt door een niet uitgeprint getal in kolom 8. Een 2 in deze kolom veroorzaakte een dubbele spatie (enkele witregel) en een 3 in deze kolom veroorzaakte een driedubbele spatie (dubbele witregel). De waarden 4-8 waren wel gedefinieerd maar nooit gebruikt. Lees hier meer over in [#159][7]

Bijvoorbeeld het volgende:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Zou dit moeten worden:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Spaties

- Spaties tussen twee karakters in de string moeten de volgende conventie gebruiken (bekijk de discussie in [#316][10]):
  - Enkele spatie voor een nieuw woord.
  - Dubbele spatie voor een nieuwe zin.
  - Driedubbele spatie voor inspringen.

Bijvoorbeeld het volgende:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Zou dit moeten worden:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Let op

Voordat je een PR maakt zorg er alsjeblieft voor dat je wijzigingen consistent zijn met de scans!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
