# Beşdarbûn

🎌
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
**Kurdî**,
[Lietuvių][LT],
[Nederlands][NL],
[Norsk][NO],
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
[DA]:CONTRIBUTING.da.md
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
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md
[TR]:CONTRIBUTING.tr.md
[UA]:CONTRIBUTING.ua.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Koda xwerû di vê depoyê de ji kaxezek çapkirî bi awak destkî hat dîjîtîzkirin, ji ber vê yekê nakokî û cudahiyên din bi şaşî hatine danîn. Pêdivîye ku kod bêne guhertin da ku li gorî çapên şandî li hev bikin:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Berfirehiyên bikêr

GitHub ji bo sazkirina zimanê AGC-ê desteka syntax heye. Mixabin redaktorê kodê yê li gel wê bê ev destek be, lê di heman dem de berfirehiyên zimanê AGC yê ku ronahî li ser syntaxê dikin ji bo redaktorên jêrîn hene:

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

† Piştgiriya formasyona otomatîk dikin

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

## Formasyon

**Nîşe:** GitHub û pêvekên ku li jor hatine nîşankirin wê tekez bikin ku hûn bi awak otomatîk pêvekirina rast bikar tînin.

- Dirêjahiya tab bikar bînin
- Dirêjahiya tab ya 8 bikar bînin
- Zêdebûna spî jê bibe

## Ez çi kontrol dikim?

Çi ciyawaziya di navbera skana û koda xwerû di vê depoyê de, di nav de:

### Şîrove

- Şîrovên di koda ku hatî veguheztin divê tam weke skanan be.
  - Ev dibe ku di nav de şaştiyek qesdî be yan jî jêbirin/zedekirina şîrovekî bi tevahî.

### Şikandina xetê

- Şikandinên xetan yê ku *li gel* `R0000` di lingê 1 de gereke bi tam weke skanan bin.
- Şikandinên xetan yê ku *ne li get* `R0000` di lingê 1 gereke tenê 1 an 2 xetên vala li peyhev hebin.
  - Eger zêdetir ji 2 şikandinê xetan hebin, şikandinê xetan yê zêde tecrîd bike.
    - Xetên li gel `R0000` di lingê 1 de di nav van de nayê jimartin.
  - Di wêneyên skanan de, ev şikandinên xetan hatine çêkirin di riya jimarek ne çapkirî di lingê 8an de. Jimara 2 li wer valahiyek dualî mecbûr kiriyê (yek xeta vala) û jimara 3 valahiyek sêalî mecbur kiriye (du xetên vala). Nirxên 4-8-ê hatin diyar kirin lê ti carî ne hatine bikaranîn. Zêdetir li ser wê bixwînin [#159][7]

Wek mînak jêrîn:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Gereke bibe:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Valahî

- Valahiyên di navbera du tîpan de di rêzikê de divê peymana jêrîn rêz bikin (nîqaşê li [#316][10] binihêrin):
  - Tenê valahiyek ji bo peyvên nû.
  - Du valahî ji bo hevokên nû.
  - Sê valahî ji bo çar-valahiyan.

Wek mînak jêrîn:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Gereke bibe:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Nîşe

Berî ku hûn PR-ê çêbikin, ji kerema xwe pê tekez bikin ku guhartinên we bi skanan re hevbeş in!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
