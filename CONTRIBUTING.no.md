# Bidra

🎌
[Čeština][CZ],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
[Nederlands][NL],
**Norsk**,
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

Kilde koden I dette «repository» var digitalisert manuelt ifra papir utskrifter, om det finnes stavefeil og andre avvik skall koden reflektere dette. Skrivefeil og andre avvik kan ha blitt innført ved et uhell men skal allikevel være identisk med følgende digitalisert papir utskrifter:

- [AGC utskrift for Comanche][8]
- [AGC utskrift for Luminary][9]

## Nytteprogrammer

GitHub har kildekode støtte for «assembly» programmerings språket AGC ( innebygget ), og de følgene tekst redigerings programmene har også støtte for nøkkel ord fargesetting:

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

† Supporterer automatisk formattering

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

**Merk:** GitHub og redigerings programmene nevnt ovenfor vil sørge for at du bruker korrekt formattering av kilde koden.

-	Bruk Tab for innrykk
-	Bruk Tab med 8 mellomrom
-	Ta bort mellomrom og annet som ikke er synlig på slutten av setninger

## Hva må jeg sjekke?

Eventuelle avvik ifra de innskannede bildene og kilde koden i dette «repository»

### Kommentarer

Kommentarer i denne koden **SKAL** stemme **eksakt** med den digitaliserte originalen.

Vanlige feil en bør se etter er blant annet:

#### Typografiske feil

Noen steder skrev original utvikleren feil i kode kommentarene. Noen av disse feilene ble rettet under digitaliseringen, og digitaliseringen har også introdusert nye typografiske feil som ikke kommer ifra papir utskriften.

For eksempel, om de originale dokumentene var feilstavet til ‘SPAECRAFT’ og de digitaliserte dokumentene ble endret til ‘SPACECRAFT’ så **SKAL** dette rettes til og reflektere de originale dokumentene, altså uten ‘C’ i ordet.

På samme måte, om digitaliseringen introduserte stavefeil som ikke var tilstede i de originale dokumentene **SKAL** dette rettes.

#### Mellomrom

Mellomrom mellom to karakterer i teksten **BØR** følge de digitaliserte dokumentene. I de fleste tilfeller (se diskusjon i [#316[10]) betyr dette:

- Enkelt mellomrom for nye ord
- Dobbelt mellomrom for nye linjer
- Trippel mellomrom for innrykk

Ikke alle sider i de digitaliserte dokumentene følger denne generaliseringen, så om de digitaliserte dokumentene bruker enkelt mellomrom i stedet for dobbelt mellomrom så skal enkelt mellomrom følges.

### Linje Skift

- Linje skift *med* `R0000` i kolonne 1 skal stemme helt med originalen.
- Linje skift *uten* `R0000` i kolonne 1 skal bare ha en eller to blanke linjer etter seg.
  - Om det er flere enn to blanke linjer skal de fjernes
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

Før en gjør en «Pull Request» bør man være helt sikker på at forandringer er helt lik de digitaliserte bildene!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
