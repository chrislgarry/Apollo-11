# Contribuire

🎌
[Čeština][CZ],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
**Italiano**
[Kurdi][KU],
[Lietuvių][LI],
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

Il codice sorgente in questa repository è stato digitalizzato manualmente da stampe su carta, quindi errori di battitura e altre differenze sono state aggiunte accidentalmente. Il codice dovrebbe essere modificato per essere consistente con le seguenti scannerizzazioni:

- [AGC stampa per Comanche][8]
- [AGC stampa per Luminary][9]

## Estensioni utili

Github supporta la sintassi dell'assembly AGC nativamente. Sfortunatamente il tuo editor di codice non lo farà, tuttabia ci sono estensioni che aggiungono la colorazione della sintassi per questi editor:

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

† Supports automatic formatting

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

## Formattazione

**Nota:** GitHub e le estenzioni riportate di sopra si assicureranno che tu stia usando la formattazione corretta automaticamente.

- Indenta con le tabulazioni
- Usa una tabulazione di larghezza 8
- Elimina spazi a fine riga

## Che cosa devo controllare?

Qualsiasi differenza tra le scannerizzazioni e il codice sorgente in questa repository.

### Commenti

Commenti nel codice trascritto **DEVONO** coincidere **esattamente** con quelli delle scannerizzazioni.

Errori comuni a cui dovresti stare attento mentre verifichi includono, ma non sono limitati a:

#### Errori tipografici

Qualche volta, gli originali sviluppatori hanno fatto degli errori tipografici mentre scrivevano i commenti. Alcuni sono stati erroneamente corretti durante l'iniziale digitalizzazione, comunque la digitalizzazione ha anche introdotto altri errori tipografici che non erano presenti nelle scannerizzazioni originali.

Per esempio, se i commenti digitalizzati contengono `SPACEFRAFT`, ma `SPAECRAFT` è stato stampato sulle scannerizzazioni, allora le digitalizzaioni **DEVONO** essere corrette in `SPAECRAFT` (senza `C`).

Analogamente, se una parola ha un errore di ma è scritta correttamente nelle scannerizzazioni allora l'errore **DEVE** essere corretto.

#### Spazi

Spazi tra due caratteri nei commenti **DOVREBBERO** corrispondere alle scannerizzazioni. Nella maggior parte dei casi (Si guardi la discussione in [#316][10]), questo comprende:

- Spazio singolo tra parole.
- Doppio spazio tra frasi.
- Triplo spazio per l'indentazione.

Non tutte le pagine nelle scannerizzazioni seguono queste generalizzare, se le scannerizzazioni hanno un solo uno spazio, usane uno solo.

### Ritorni a capo

- I ritorni a capo *con* `R0000` nella colonna 1 dovrebbero corrispondere alle scannerizzaioni esattamente.
- I ritorni a capo *senza* `R0000` nella colonna 1 dovrebbero contenere solo 1 o 2 linee vuote di seguito.
  - Se ci sono più di 2 ritorni a capo, elimina quelli in eccesso.
    - Righe con `R0000` nella colonna 1 non seguono questa regola.
  - Nelle immagini originali, queste sono state create da un cifra non stampato nella colonna 8. Un 2 lì ha forzato un duppio spazio (una riga vuota), invece un 3 ha forzato unn spazio triplo (due righe vuote). Valori da 4 a 8 sono
  stati definiti ma non sono mai stati usati. Puoi leggere di più qui [#159][7]

Per esempio, il seguente:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Dovrebbe diventare:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Note

Prima di fare una PR(Pull Request), per piacere controlla che i tu tuoi cambiamenti siano consistenti con le scannerizzazioni!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
