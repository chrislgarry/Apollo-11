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
[SV]:CONTRIBUTING.sv.md
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Källkoden i det här kodarkivet digitaliserades manuellt från pappersutskrifter, så stavfel och andra avvikelser har införts av misstag. Koden ska modifieras så att den överensstämmer med följande skannade utskrifter:

- [AGC-utskrifter för Comanche][8]
- [AGC-utskrifter för Luminary][9]

Följande webbplats kan användas för att enkelt navigera runt de skannade utskrifterna för både Comanche och Luminary: https://28gpc.csb.app/

## Användbara tillägg

GitHub har syntaxstöd för AGC-assemblerspråket inbyggt. Tyvärr har inte din kodredigerare det, men det finns AGC-språktillägg som ger syntaxmarkering för följande redigerare:

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

† Stöder automatisk formatering

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

**Notering:** GitHub och tillägg markerade ovan säkerställer att du använder rätt formatering automatiskt.

- Använd flikindrag
- Använd flikbredden 8
- Trimma efterföljande blanksteg

## Vad ska jag kolla?

Eventuella avvikelser mellan skanningarna och källkoden i det här kodarkivet.

### Kommentarer

Kommentarer i den transkriberade koden **MÅSTE** matcha skanningarna **exakt**.

Vanliga problem som du bör hålla utkik efter vid korrekturbehandling inkluderar, men inte begränsat till:

#### Typografiska fel

På vissa ställen gjorde de ursprungliga utvecklarna typografiska fel när de skrev kommentarer. En del av dessa korrigerades av misstag under den inledande digitaliseringen, men digitaliseringen har också infört typografiska fel som inte fanns i skanningarna.

Till exempel, om de digitaliserade kommentarerna innehöll "SPACECRAFT", men "SPAECRAFT" skrevs ut i skanningarna, då **MÅSTE* korrigeras till "SPAECRAFT" (saknas "C").

Likaså, om ett ord har ett stavfel i digitaliseringen men är rättstavat i skanningarna så **MÅSTE** stavfelet korrigeras.

#### Mellanslag

Mellanslag mellan två tecken i kommentarerna **SKA** matcha skanningarna. I de flesta fall (se diskussionen i [#316][10]) är detta:

- Ett mellanslag för nya ord.
- Dubbelt mellanslag för nya meningar.
- Trippelt mellanslag för fördjupningar.

Alla sidor i skanningarna följer inte denna generalisering, om skanningarna bara har ett enda mellanslag istället för ett dubbelt mellanslag, använd ett enda mellanslag.

### Radbrytningar

- Radbrytningar *med* `R0000` i kolumn 1 bör matcha skanningarna exakt.
- Radbrytningar *med**__out__* `R0000` i kolumn 1 bör endast innehålla 1 eller 2 tomma rader i rad.
  - Om det finns fler än 2 tomma radbrytningar, ta bort de extra radbrytningarna.
    - Rader med `R0000` i kolumn 1 räknas inte till detta.
  - I källbilderna skapades dessa av en otryckt siffra i kolumn 8. En 2:a tvingade fram ett dubbelt mellanslag (enkel blank rad) och en 3:a tvingade fram ett trippelt mellanslag (dubbel blank linje). Värdena 4-8 definierades men användes aldrig. Läs mer om det i [#159][7]

Till exempel följande:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Bör bli:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Notering

Innan du gör en PR, se till att dina ändringar överensstämmer med skanningarna!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
