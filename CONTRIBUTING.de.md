# Contributing

🎌
[Català][CA]،
[Čeština][CZ],
[Dansk][DA],
**Deutsch**,
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Türkçe][TR],
[Ελληνικά][GR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CA]:CONTRIBUTING.ca.md
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
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Der Quellcode in diesem Repository wurde manuell digitalisiert, also sind jegliche Tippfehler und Diskrepanzen unabsichtlich eingeführt worden. Der Code soll so angepasst werden, dass er mit den folgenden eingescannten Ausdrucken übereinstimmt:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Nützliche Erweiterungen

GitHub verfügt über eine integrierte Syntax-Unterstützung für die AGC Assembly Sprache. Das gilt leider nicht für deinen Code Editor. Für die folgenden Editoren gibt es aber AGC-Spracherweiterungen, die Syntaxhervorhebung hinzufügen:

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

† Unterstützt automatische Formatierung

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

## Formatierung

**Anmerkung:** GitHub und die oben erwähnten Erweiterungen werden automatisch sicherstellen, dass du die korrekte Formatierung einhältst.

- Verwende Tab-Einrückung
- Verwende Tab-Breite von 8
- Entferne Leerzeichen am Ende einer Zeile

## Was soll ich überprüfen?

Alle Diskrepanzen zwischen den Scans und dem Quelltext in diesem Repository.

### Kommentare

Kommentare in dem transkribierten Quellcode **MÜSSEN** denen in den Scans **genau** entsprechen.

Häufige Fehler, auf die du dich unter anderem beim Korrekturlesen achten solltest sind folgende (nicht abschließend):

#### Typographische Fehler

An einigen Stellen haben die ursprünglichen Entwickler typographische Fehler beim Schreiben von Kommentaren gemacht. Manche von diesen wurden fälschlicherweise bei der ursprünglichen Digitalisierung korrigiert, die Digitalisierung hat jedoch auch typografische Fehler eingeführt die nicht in den Scans vorhanden sind.

Wenn die digitalisierten Kommentare zum Beispiel `SPACECRAFT` enthielten aber `SPAECRAFT` in den Scans stand, dann **MUSS** die Digitalisierung zu `SPAECRAFT` korrigiert werden (fehlendes `C`).

Gleichermaßen gilt, dass wenn ein Wort einen Tippfehler in der Digitalisierung aufweist aber in den Scans korrekt geschrieben wurde der Tippfehler korrigiert werden **MUSS**.

#### Leerzeichen

Leerzeichen zwischen zwei Zeichen in Kommentaren **SOLLTEN** den Scans angeglichen werden. In den meisten Fällen (siehe die Diskussion in [#316][10]) sind es:

- Einzelne Leerzeichen für neue Wörter.
- Doppelte Leerzeichen für neue Sätze.
- Dreifache Leerzeichen für Einrückungen.

Nicht alle Seiten in den Scans befolgen diese Verallgemeinerung, wenn es in den Scans nur ein einzelnes Leerzeichen gibt statt einem doppelten Leerzeichen, verwende ein einzelnes Leerzeichen.

### Zeilenumbrüche

- Zeilenumbrüche *mit* `R0000` in Spalte 1 sollten genau mit den Scans übereinstimmen
- Zeilenumbrüche *__ohne__* `R0000` in Spalte 1 sollten nur 1 bis 2 Leerzeilen hintereinander haben
  - Wenn es mehr als 2 Leerzeilen gibt, entferne die zusätzlichen Zeilenumbrüche.
    - Zeilen mit `R0000` in Spalte 1 werden dabei nicht dazugezählt.
  - In den Originalbildern wurden diese durch eine nicht gedruckte Ziffer in Spalte 8 verursacht. Eine 2 an dieser Stelle erzwang ein doppeltes Leerzeichen (einzelne Leerzeile) und eine 3 erzwang ein dreifaches Leerzeichen (doppelte Leerzeile). Die Werte zwischen 4 und 8 waren definiert, wurden aber nie verwendet. Mehr dazu in [#159][7]

Zum Beispiel, das folgende:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Sollte werden zu:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Anmerkung

Bevor du einen PR erstellst, stelle bitte sicher, dass deine Änderungen mit den Scans übereinstimmen!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
