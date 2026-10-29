# Bidrag

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

Kildekoden i dette «repository» er digitaliseret manuelt fra papirudskrifter, så skrivefejl og andre afvigelser kan være blevet introduceret. Koden skal modificeres så det er konsistent med følgende indskannede udskrifter:

- [AGC udskrift for Comanche][8]
- [AGC udskrift for Luminary][9]

Følgende hjemmeside kan bruges til nemt at navigere rundt i de scannede udskrifter for både Comanche og Luminary: https://28gpc.csb.app/

## Brugbare udvidelser

GitHub har indbygget syntaks understøttelse for AGC assembler sprog. De fleste kodeeditorer har ikke, men der findes AGC sprog udvidelser som giver syntaks fremhævelse for følgende editorer:

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

† Supportere automatisk formattering

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

## Formattering

**Note:** GitHub og udvidelser markeret ovenfor vil sikre at du bruger den rigtige formattering automatisk.

- Brug tab indrykning
- Brug tab bredde svarende til 8 mellemrum
- Fjern mellemrum, tab og andre tegn der ikke er synlig fra slutningen af linien

## Hvad skal jeg kontrollere?

Alle forskelle mellem det indskannede og kildekoden i «repositoriet».

### Kommentarer

Kommentarer i den transskiberede kildekode **skal** matche det indskannede **fuldstændigt**.

Ofte forekomne forskelle som du skal kigge efter indeholder, men er ikke begrænset til:

#### Typografiske fejl

Nogle steder har de oprindelige udviklere lavet typegrafiske fejl da de skrev kommentarer. Nogle af disse blev fejlagtigt rettet under den oprindelige digitalisering, men selve digitaliseringen har også introduceret typografiske fejl, som ikke var til stede i indskanningerne.

For eksempel, hvis de digitaliserede kommentarer indeholdt `SPACECRAFT`, men der stod `SPAECRAFT` i det indskannede, så **SKAL** det rettes til `SPAECRAFT` (mangler `C`).

På samme måde, hvis et ord har en stavefejl i digitaliseringen, men er stavet korrekt i det indskannede, så **SKAL** stavefejlen rettes.

#### Mellemrum

Mellemrum mellem to tegn i kommentarer **BØR** matche det indskannede. I de fleste tilfælde (se diskussionen i [#316][10]) gælder følgende:

- Enkel mellemrum ved nyt ord.
- Dobbel mellemrum for en ny sætning.
- Trippel mellemrum for indrykning.

Ikke alle sider in det indskannede følger denne generalisering. Hvis det indskannede kun bruger et enkelt mellemrum i stedet for dobbel mellemrum, brug et enkelt mellemrum.

### Linie skift

- Linie skift *med* `R0000` i kolonne 1 bør matche det indskannede præcist.
- Linie skift *uden* `R0000` i kolonne 1 bør kun indeholde 1 eller 2 blanke linier i træk.
  - Hvis der er mere end 2 blank linie skift, fjern de ekstra linie skift.
    - Linier med `R0000` i kolonne 1 skal ikke tælles med.
  - I originalen, dette blev skabt af en tegn i kolonne 8, som ikke blev skrevet ud. Stod der et 2-tal blev det til et dobbel mellemrum (enkel blank linie) og et 3-tal blive til trippel mellemrum (dobbel blank linie). Værdierne 4-8 var defineret, men blev aldrig brugt. Læs mere om det i [#159][7]

For eksempel skal følgende koden:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Se således ud:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Note

Før du laver en PR, sørg venligst for at dine ændringer er konsistente med det indskannede!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
