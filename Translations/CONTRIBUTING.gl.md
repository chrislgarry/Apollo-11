# Contribuíndo

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

O código fonte neste repositorio dixitalizouse manualmente a partir de impresións en papel, polo que os erros tipográficos e outras discrepancias foron introducidas accidentalmente. O código modificarase para que sexa coherente coas impresións escaneadas:

- [AGC impresións para Comanche][8]
- [AGC impresións para Luminary][9]

O seguinte sitio web pódese usar para navegar facilmente polas impresións escaneadas tanto para Comanche como para Luminary: https://28gpc.csb.app/

## Extensións Útiles

GitHub ten soporte de sintaxis para o lenguaxe ensamblador AGC incorporado. Lamentablemente, o seu editor de código non o fará, non obstante, hai extensións de linguaxe AGC que proporcionan resaltado de sintaxis para os seguintes editores:

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

† Admite formatado automático

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

## Formatado

**Nota:** GitHub e as extensións marcadas anteriormente aseguraranse de que estés utilizando o formato correcto automaticamente.

- Empregar sangría de separación
- Empregar un ancho de pestana de 8
- Recortar espazos en branco ao final

## Que comprobar?

Calqueira discrepancia entre os escaneos e o código fonte neste repositorio, incluíndo:

### Comentarios

- Os comentarios no código transcrito deben coincidir exactamente cos escaneos
  - Isto podería implicar crear un erro tipográfico deliberado ou eliminar / agregar un comentario completo.

### Saltos de liña

- Salto de liña *with* `R0000` na columna 1 debe coincidir exactamente cos escaneos.
- Salto de liña *with**__out__* `R0000` na columna 1 debe conter só 1 ou 2 liñas en branco nunha fila.
  - Se hai máis de 2 saltos de líña en branco, elimine os saltos de líña adicionales.
    - Liñas con `R0000` na columna 1 non contan para isto.
  - Nas imaxes de orixen, estas foron creadas por un díxito sen imprimir na columna 8. A 2 forzou un dobre espazo (liña en branco sinxela) e un 3 forzou un espazo triple (liña en branco doble). Os valores 4-8 definíronse pero nunca se empregaron. Lea máis sobre isto en [#159][7]

Por exemplo o siguiente:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Debe convertirse:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Espazos

- Os espazos entre dous caracteres na cadea deben respetar a seguinte convención (vexa a discusión en [#316][10]):
  - Espazo único para novas palabras.
  - Dobre espazo para novas oracións.
  - Triple espazo para fendas.

Por exemplo o seguiente:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Debe convertirse:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Nota

Antes de facer un RP, asegúrese de que os seus cambios sexan consistentes cos escaneos!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
