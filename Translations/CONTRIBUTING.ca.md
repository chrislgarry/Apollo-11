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

El codi font en aquest repositori es va digitalitzar manualment a partir d'impressions en paper, d'aquesta manera els errors tipogràfics i altres discrepàncies s'han introduït accidentalment. El codi es modificarà perquè sigui coherent amb les impressions escanejades:

- [AGC impressions per a Comanche][8]
- [AGC impressions per a Luminary][9]

El següent lloc web es pot utilitzar per navegar fàcilment per les impressions escanejades tant per a Comanche com per a Luminary: https://28gpc.csb.app/

## Extensions Útils

GitHub té suport de sintaxi pel llenguatge assemblador AGC incorporat. Malauradament, el seu editor de codi no ho farà, no obstant això, hi ha extensions de llenguatge AGC que proporcionen ressaltat de sintaxi pels següents editors:

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

† Admet formateig automàtic

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

## Formateig

**Nota:** GitHub i les extensions marcades anteriorment asseguren que s'està utilitzant el format correcte automàticament.

- Utilitzar la tabulació per la indentació
- Utilitza una tabulació de 8
- Retalla espais en blanc al final

## ¿Que verifico?

Qualsevol discrepància entre els escanejos i el codi font d'aquest repositori, incloent-hi:

### Comentaris

Els comentaris en el codi transcrit **han de coincidir exactament** amb els escanejos

Això podria implicar crear un error tipogràfic deliberat o eliminat/ agregar un comentari complet.

Els problemes més habituals que heu de tindre en compte durant la prova inclouen, entre d'altres:

### Salts de línia

- Salt de línia *with* `R0000` en la columna 1 ha de coincidir exactament amb els escanejos.
- Salt de línia *with**__out__* `R0000` en la columna 1 hauria de contenir sols 1 o 2 línies en blanc en una fila.
  - Si hi ha més de 2 salts de línia en blanc, elimina els salts de línia addicionals.
    - Línies amb `R0000` en la columna 1 no conten per això.
  - En les imatges d'origen, aquestes van ser creades per un dígit sense imprimir en la columna 8. A 2 va forçar un doble espai (línia amb blanc simple) i un 3 va forçar un espai triple (línia en blanc doble). Els valors 4-8 es van diferenciar però mai es van utilitzar. Llegeix més sobre  això a [#159][7]

Per exemple el següent:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

S'ha de convertir en:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Espais

- Els espais entre dos caràcters en la cadena han de respectar la següent convenció (observa la discussió a [#316][10]):
  - Espai únic per noves paraules.
  - Doble espai per noves oracions.
  - Triple espai per esquerdes.

Per exemple el següent:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

S'ha de convertir en:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Nota

Abans de fer una RP, ¡assegurat que els seus canvis siguin consistents amb els escenaris!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
