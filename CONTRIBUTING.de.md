# Contributing

:crossed_flags:
**Deutsch**,
[English][EN],
[Español][ES],
[Français][FR],
[Nederlands][NL],
[Português][PT_BR],
[Türkçe][TR],
[العربية][AR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[DE]:CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[NL]:CONTRIBUTING.nl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Der Quellcode in diesem repository wurde manuell digitalisiert, also sind jegliche schriftliche Fehler und Ungereimtheiten aus versehen hinzugefügt worden. Der Code soll Modifiziert werden so dass er mit den Vorlagen übereinstimmt. 

* [AGC printouts for Comanche][8]
* [AGC printouts for Luminary][9]

## Nützliche Erweiterungen

GitHub hat eine Integrierte Unterstützung für AGC assembly. Dasselbe gilt für ihren Code Editor nicht, dennoch gibt es AGC Erweiterungen die Syntax highlighting für die folgenden Editors zur Verfügung stellen: 
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

† Unterstützen Automatisches Formatieren  

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
**Anmerkung:** GitHub und die oben erwähnten Erweiterungen werden automatisch sicherstellen dass sie die Korrekte Formatierung einhalten.

- benutze tab indentation
- benutze tab width von 8
- entferne nachlaufende leerstellen 

## Was sol ich überprüfen  ?
Jegliche Ungereimtheiten zwischen den scans und den Code in die repository, einschließlich: 

### Kommentare 
- Kommentare in dem Abgeschriebenen Code müssen denen in den scans gleichen
  - Das könnte extra einen sprachlichen Fehler oder das entfernen/hinzufügen eines ganzen Kommentars bedeuten 
  
### Zeilenumbruch
- Zeilenumbruch *mit* `R0000` in Spalte 1 sollte genau mit den scans übereinstimmen 
- Zeilenumbruch *mit**__out__* `R0000` in Spalte 1 sollte nur 1 bis zwei Leerzeilen nacheinander haben
  - Wenn dort mher als 2 leere Zeilenumbrüche sind sollen diese entfernt werden
    - Zeilen mit `R0000` in Spalte 1 zählen nicht dazu 
  - In den Quell Bildern, Diese wurden verursacht durch eine nicht gedruckte stelle in in Spalte 8. A 2 dort hat eine doppelte Leerstelle (einezilne Leerzeile) und A 3 hat eine dreifache Leerstelle (doppelte Leerzeile). Werte 4-8 wurden definiert aber nie benutzt. Mehr hier zu in [#159][7] 
  
zum Beispiel das Folgende:
```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```
sollte werden:
```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Hinweis

Bevor sie eine PR starten, bitte überprüfen sie ob die Veränderungen mit den Scans über einstimmen!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741


