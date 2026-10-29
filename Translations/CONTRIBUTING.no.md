# Bidra

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

Kildekoden i dette depotet ble digitalisert manuelt fra papirutskrifter, skrivefeil og andre avvik kan ha blitt innført ved et uhell. Koden skal modifiseres til å være identisk med følgende digitaliserte papirutskrifter:

- [AGC-utskrift for Comanche][8]
- [AGC-utskrift for Luminary][9]

Følgende nettside er lett tilgjengelig med skannede utskrifter av både Comanche og Luminary: https://28gpc.csb.app/

## Nytteprogrammer

GitHub har innebygd støtte for programmeringsspråket AGC "assembly", og følgene tekstredigeringsprogrammer har også utvidelser som gir støtte for syntaksutheving i AGC:

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

† Støtter automatisk formatering

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

## Formatering

**Merk:** GitHub og de markerte redigeringsprogrammene vil sørge for at du bruker korrekt formatering av kildekoden automatisk.

-	Bruk Tab for innrykk
-	Bruk Tab med 8 mellomrom
-	Ta bort mellomrom og annet som ikke er synlig på slutten av setninger

## Hva må jeg sjekke?

Eventuelle avvik fra de innskannede bildene og kildekoden i dette depotet.

### Kommentarer

Kommentarer i denne koden **SKAL** stemme **eksakt** med den digitaliserte originalen.

Vanlige feil man bør se etter er blant annet:

#### Typografiske feil

Noen steder skrev originalutviklerne feil i kodekommentarene. Noen av disse skrivefeilene ble feilaktig rettet under digitaliseringen, og digitaliseringen har også introdusert nye skrivefeil som ikke stammer fra papirutskriften.

For eksempel, hvis de originale dokumentene var feilstavet som ‘SPAECRAFT’ og de digitaliserte dokumentene ble endret til ‘SPACECRAFT’ så **SKAL** dette rettes for å reflektere de originale dokumentene, altså manglende ‘C’.

På samme måte, hvis digitaliseringen introduserte stavefeil som ikke var tilstede i de originale dokumentene **SKAL** dette rettes.

#### Mellomrom

Mellomrom mellom to tegn i teksten **BØR** følge de digitaliserte dokumentene. I de fleste tilfeller (se diskusjon i [#316[10]) betyr dette:

- Enkelt mellomrom for nye ord
- Dobbelt mellomrom for nye linjer
- Trippel mellomrom for innrykk

Ikke alle sider i de digitaliserte dokumentene følger denne generaliseringen, hvis de digitaliserte dokumentene bruker enkelt mellomrom i stedet for dobbelt mellomrom, skal enkelt mellomrom brukes.

### Linjeskift

- Linje skift *med* `R0000` i kolonne 1 skal stemme helt med originalen.
- Linje skift *uten* `R0000` i kolonne 1 skal bare ha en eller to blanke linjer etter seg.
  - Hvis det er flere enn to blanke linjer skal de fjernes
    - Linjer med `R0000` i kolonne 1 regnes ikke med i slike tilfeller
  - I originalen ble disse skapt av et siffer i kolonne 8 (som ikke ble skrevet ut). et 2-tall fremtvang dobbelt mellomrom (enkel blank linje) og et 3-tall så ble det et trippel mellomrom (to doble linjer). Verdiene fra 4 til 8 var definert, men ikke brukt. Les mer om dette i [#159][7]

For eksempel skal følgende kode:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Se slik ut:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Noter

Før man åpner en «Pull Request», bør man være helt sikker på at endringene er i samsvar med de digitaliserte bildene!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
