# Contributing

🎌
[Čeština][CZ],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Kurdi][KU],
[Nederlands][NL],
[Português][PT_BR],
**Polski**,
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
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[NL]:CONTRIBUTING.nl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[PL]:CONTRIBUTING.pl.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Kod źródłowy w tym repozytorium został zdigitalizowany ręcznie na podstawie wydruków papierowych, więc literówki i inne rozbieżności zostały wprowadzone przypadkowo. Kod należy zmodyfikować, aby był zgodny z zeskanowanymi wydrukami:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Przydatne rozszerzenia

GitHub obsługuje składnię wbudowanego języka asemblera AGC. Niestety twój edytor kodu prawdopodobnie nie będzie go obsługiwał, jednak istnieją rozszerzenia języka AGC, które zapewniają obsługę składni tego języka dla następujących edytorów:

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

† Wspiera automatyczne formatowanie

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

## Formatowanie

**Uwaga:** GitHub wraz z powyższymi rozszerzeniami zapewniają automatyczne użycie prawidłowego formatowana

- Tabulacja
- Szerokość tabulacji wynosząca 8
- Niepozostawianie spacji na końcu wierszy

## Co mam sprawdzać?

Wszelkie rozbieżności między skanami a kodem źródłowym tego repozytorium, w tym:

### Komentarze

- Komentarze w transkrybowanym kodzie powinny dokładnie odpowiadać skanom
  - Obejmuje to korekcje popełnionych błędów lub usunięcie / dodanie całego komentarza.

### Rozdzielanie wierszy

- Wiersz *rozpoczynający się od* `R0000` w kolumnie 1 powinien dokładnie pasować do skanów.
- Wiersz *nierozpoczynający się od* `R0000` w kolumnie 1 powinien zawierać tylko 1 lub 2 puste wiersze z rzędu.
  - Jeżeli są więcej niż 2 puste wiersze - usuń ich nadmiar.
    - Wiersze rozpoczynające się od `R0000` w kolumnie 1 nie są zaliczane jako pusty wiersz.
  - W obrazach źródłowych puste linie zostały utworzone przez niezadrukowaną cyfrę w kolumnie 8. 2 wymusiło podwójną spację (pojedyncza pusta linia), a 3 wymusiło potrójną spację (podwójna pusta linia). Wartości 4-8 zostały zdefiniowane, ale nigdy nie były używane. Więcej na ten temat w [#159][7]

Na przykład:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Zostaje zmienione na:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Spacje

- Spacje pomiędzy dwoma znakami w danym ciągu powinny być zgodne z następującą konwencją (patrz na [#316][10]):
  - Pojedyńcza spacja dla nowego słowa.
  - Podwójna spacja dla nowego zdania.
  - Potrójna spacja dla akapitu.

Na przykład:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Zostaje zmienione na:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Uwaga

Zanim utworzysz PR, upewnij się, że zmiany są zgodne ze skanami!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
