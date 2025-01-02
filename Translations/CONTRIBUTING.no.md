# Bidra

🌐
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

Kildekoden i dette depotet ble digitalisert manuelt fra papirutskrifter, hvis det finnes stavefeil og andre avvik skal koden reflektere dette. Skrivefeil og andre avvik kan ha blitt innført ved et uhell, men skal likevel være identisk med følgende digitaliserte papirutskrifter:

- [AGC-utskrift for Comanche][8]
- [AGC-utskrift for Luminary][9]

Følgende nettside er lett tilgjengelig med skannede utskrifter av både Comanche og Luminary: https://28gpc.csb.app/

## Nytteprogrammer

GitHub har innebygd støtte for programmeringsspråket AGC "assembly", og følgene tekstredigeringsprogrammer har også støtte for syntaksutheving:

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

**Merk:** GitHub og de nevnte redigeringsprogrammene vil sørge for at du bruker korrekt formatering av kildekoden.

-	Bruk Tab for innrykk
-	Bruk Tab med 8 mellomrom
-	Ta bort mellomrom og annet som ikke er synlig på slutten av setninger

## Hva må jeg sjekke?

Eventuelle avvik fra de innskannede bildene og kildekoden i dette depotet

### Kommentarer

Kommentarer i denne koden **SKAL** stemme **eksakt** med den digitaliserte originalen.

Vanlige feil man bør se etter er blant annet:

#### Typografiske feil

Noen steder skrev originalutvikleren feil i kodekommentarene. Noen av disse feilene ble rettet under digitaliseringen, og digitaliseringen har også introdusert nye typografiske feil som ikke stammer fra papirutskriften.

For eksempel, hvis de originale dokumentene var feilstavet som ‘SPAECRAFT’ og de digitaliserte dokumentene ble endret til ‘SPACECRAFT’ så **SKAL** dette rettes for å reflektere de originale dokumentene, altså uten ‘C’ i ordet.

På samme måte, om digitaliseringen introduserte stavefeil som ikke var tilstede i de originale dokumentene **SKAL** dette rettes.

#### Mellomrom

Mellomrom mellom to tegn i teksten **BØR** følge de digitaliserte dokumentene. I de fleste tilfeller (se diskusjon i [#316[10]) betyr dette:

- Enkelt mellomrom for nye ord
- Dobbelt mellomrom for nye linjer
- Trippel mellomrom for innrykk

Ikke alle sider i de digitaliserte dokumentene følger denne generaliseringen, så hvis de digitaliserte dokumentene bruker enkelt mellomrom i stedet for dobbelt mellomrom så skal enkelt mellomrom følges.

### Linjeskift

- Linjeskift *med* `R0000` i kolonne 1 skal stemme helt med originalen.
- Linjeskift *uten* `R0000` i kolonne 1 skal bare ha en eller to blanke linjer etter seg.
  - Om det er flere enn to blanke linjer, skal de fjernes
    - Linjer med `R0000` i kolonne 1 regnes ikke i slike tilfeller
  - I originalen ble disse skapt av et uskrevet tegn i kolonne 8. Sto det 2 der var der et dobbelt mellomrom ( enkel blank linje ) og om det var 3 så ble det et trippel mellomrom ( to dobble linjer ). Verdiene ifra 4 til 8 var definert, men er ikke i bruk. Les mer om dette i [#159][7]

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