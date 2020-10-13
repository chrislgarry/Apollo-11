# Contributing

🎌
[Čeština][CZ],
**Deutsch**,
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
[Nederlands][NL],
[Norsk][NO],
[Português][PT_BR],
[Türkçe][TR],
[Ελληνικά][GR],
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
[GR]:CONTRIBUTING.gr.md
[IT]:CONTRIBUTING.it.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[LT]:CONTRIBUTING.lt.md
[NL]:CONTRIBUTING.nl.md
[NO]:CONTRIBUTING.no.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Der Quellcode in diesem Repository wurde manuell digitalisiert, also sind jegliche schriftlichen Fehler und Ungereimtheiten aus Versehen hinzugefügt worden. Der Code soll so modifiziert werden, dass er mit den Vorlagen übereinstimmt.

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Nützliche Erweiterungen

GitHub hat eine integrierte Unterstützung für AGC assembly. Das gilt leider nicht für deinen Editor. Für die folgenden Editoren gibt es aber AGC-Erweiterungen, die Syntaxhighlighting hinzufügen:

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

† unterstützt auch automatisches Formatieren

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

- benutze tab indentation
- benutze tab width von 8
- entferne Leerzeichen am Ende einer Zeile

## Was soll ich überprüfen?

Alle Ungereimtheiten zwischen den Scans und dem Code in diesem Repository, einschließlich:

### Kommentare

- Kommentare in dem abgeschriebenen Code müssen denen in den Scans gleichen
  - Das kann bedeuten, dass man einen Rechtschreibfehler übernehmen oder einen ganzen Kommentar entfernen/hinzufügen muss

### Zeilenumbruch

- Zeilenumbrüche *mit* `R0000` in Spalte 1 sollte genau mit den Scans übereinstimmen
- Zeilenumbrüche *__ohne__* `R0000` in Spalte 1 sollten nur ein bis zwei Leerzeilen nacheinander haben
  - Wenn es dort mehr als zwei Leerzeilen gibt, sollten die zusätzlichen Leerzeilen entfernt werden.
    - Zeilen mit `R0000` in Spalte 1 zählen nicht dazu
  - In den Originalbildern wurden die durch eine nicht gedruckte Ziffer in Spalte 8 verursacht. Eine 2 dort hat ein doppeltes Leerzeichen (einzelne Leerzeile) und eine 3 hat ein dreifaches Leerzeichen (doppelte Leerzeile). Die Werte 4-8 wurden definiert, aber nie benutzt. Mehr dazu hier: [#159][7]

Beispiel:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

wird zu:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Hinweis

Bevor du einen PR startest, überprüfe bitte, ob die Änderungen mit den Scans übereinstimmen!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
