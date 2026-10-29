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
[Русский][RU],
[Svenska][SV],
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
[RU]:CONTRIBUTING.ru.md
[SV]:CONTRIBUTING.sv.md
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Kod źródłowy w tym repozytorium został zdigitalizowany ręcznie na podstawie wydruków papierowych, więc literówki i inne rozbieżności zostały wprowadzone przypadkowo. Kod należy zmodyfikować, aby był zgodny z zeskanowanymi wydrukami:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Poniższą stronę internetową można łatwo przeglądać, korzystając ze zeskanowanych wydruków Comanche i Luminary: https://28gpc.csb.app/

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

Wszelkie rozbieżności między skanami a kodem źródłowym tego repozytorium.

### Komentarze

Komentarze w transkrybowanym kodzie **MUSZĄ DOKŁADNIE** odpowiadać skanom.

Typowe problemy, na które należy zwrócić uwagę podczas sprawdzania, obejmują między innymi:

#### Literówki

W niektórych miejscach pierwotni programiści popełnili błędy typograficzne podczas pisania komentarzy. Niektóre z nich zostały błędnie poprawione podczas wstępnej digitalizacji, jednak digitalizacja wprowadziła również błędy typograficzne, których nie było w skanach.

Na przykład, jeśli zdigitalizowane komentarze zawierały słowo „SPACECRAFT”, ale na skanach odczytano „SPAECRAFT”, wówczas digitalizacja **MUSI** zostać poprawiona na „SPAECRAFT” (brakujące „C”).

Podobnie, jeśli słowo zawiera literówkę w digitalizacji, ale jest poprawnie napisane na skanach, wówczas literówka **MUSI** zostać poprawiona.

#### Spacje

Spacje pomiędzy dwoma znakami w danym ciągu powinny być zgodne z następującą konwencją (patrz na [#316][10]):

- Pojedyncza spacja dla nowego słowa.
- Podwójna spacja dla nowego zdania.
- Potrójna spacja dla akapitu.

Nie wszystkie strony w skanach są zgodne z tym uogólnieniem, jeśli skany mają tylko jedną spację zamiast podwójnej, użyj jednej spacji.

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
