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
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

El código fuente en este repositorio se digitalizó manualmente a partir de impresiones en papel, por lo que los errores tipográficos y otras discrepancias se han introducido accidentalmente. El código se modificará para que sea coherente con las impresiones escaneadas:

- [AGC impresiones para Comanche][8]
- [AGC impresiones para Luminary][9]

Se puede utilizar el siguiente sitio web para navegar fácilmente por las impresiones escaneadas de Comanche y Luminary: https://28gpc.csb.app/

## Extensiones Útiles

GitHub tiene soporte de sintaxis para el lenguaje ensamblador AGC incorporado. Lamentablemente, su editor de código no lo hará, sin embargo, hay extensiones de lenguaje AGC que proporcionan resaltado de sintaxis para los siguientes editores:

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

† Admite formateo automático

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

## Formateo

**Nota:** GitHub y las extensiones marcadas anteriormente asegurarán que esté utilizando el formato correcto automáticamente.

- Use tabulación tipo sangría
- Use un ancho de pestaña de 8
- Recortar espacios en blanco al final

## ¿Qué verifico?

Cualquier discrepancia entre los escaneos y el código fuente en este repositorio, incluyendo:

### Comentarios

- Los comentarios en el código transcrito deben coincidir exactamente con los escaneos
  - Esto podría implicar crear un error tipográfico deliberado o eliminar / agregar un comentario completo.

### Saltos de línea

- Salto de línea *with* `R0000` en la columna 1 debe coincidir exactamente con los escaneos.
- Salto de línea *with**__out__* `R0000` en la columna 1 debe contener solo 1 ó 2 líneas en blanco en una fila.
  - Si hay más de 2 saltos de línea en blanco, elimine los saltos de línea adicionales.
    - Líneas con `R0000` en la columna 1 no cuentan para esto.
  - En las imágenes de origen, éstas fueron creadas por un dígito sin imprimir en la columna 8. A 2 forzó un doble espacio (línea en blanco simple) y un 3 forzó un espacio triple (línea en blanco doble). Los valores 4-8 se definieron pero nunca se usaron. Lea más sobre esto en [#159][7]

Por ejemplo lo siguiente:

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

### Espacios

- Los espacios entre dos caracteres en la cadena deben respetar la siguiente convención (vea la discusión en [#316][10]):
  - Espacio único para nuevas palabras.
  - Doble espacio para nuevas oraciones.
  - Triple espacio para hendiduras.

Por ejemplo lo siguiente:

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

Antes de hacer un RP, ¡asegúrese de que sus cambios sean consistentes con los escaneos!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
