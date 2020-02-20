# Contribuindo

:crossed_flags:
[English][EN],
[Español][ES],
**Português**,
[Türkçe][TR],
[العربية][AR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[EN]:CONTRIBUTING.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

O código-fonte neste repositório foi digitado manualmente a partir de impressões em papel, logo alguns erros de digitação e discrepâncias foram introduzidos acidentalmente. O código deve ser modificado para se tornar consistente com as seguintes digitalizações:

* [Impressões em AGC para Comanche][8]
* [Impressões em AGC para Luminary][9]

## Extensões úteis

Github possui suporte de sintaxe nativo para a linguagem assembly AGC.
Infelizmente, seu editor de texto não o terá. Portanto, existem extensões para a linguagem AGC que providenciam destaque de sintaxe para os seguintes editores:
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

† Suporta formatação automática

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

## Formatando
**Nota:** Github e as três extensões listadas acima garantirão que você estará usando a formatação correta automaticamente.

- Use indentação por tab
- Use largura de tab em 8
- Diminua o espaço em branco à direita

## O que devo conferir?
Quaisquer discrepâncias entre as digitalizações e o código-fonte neste repositório, incluindo:

### Comentários
- Comentários no código transcrito devem corresponder exatamente às digitalizações
  - Isso pode envolver criar um erro de digitação deliberadamente ou remover/adicionar um comentário inteiro.

### Quebra de linha
- Quebras de linha *com* `R0000` na coluna 1 devem corresponder exatamente às digitalizações.
- Quebras de linha *sem* `R0000` na coluna 1 devem conter apenas uma ou duas linhas em branco em sequência.
  - Se existem mais de duas quebras de linha em branco, remova as quebras de linha extras.
    - Linhas com `R0000` na coluna 1 não contam para este propósito.
  - As imagens fonte foram criadas por meio de um dígito não-impresso na coluna 8. Um 2 forçava um espaço duplo (linha em branco única) e um 3 forçava um espaço triplo (linha em branco dupla). Os valores de 4 até 8 foram definidos porém nunca usados. Leia mais sobre em [#159][7]

Por exemplo, o seguinte código:
```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```
Deve tornar-se:
```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Espaços
- Espaços entre dois caracteres na cadeia de caracteres devem respeitar a seguinte convenção (veja a discussão em [#316][10]):
  - Espaço único para novas palavras.
  - Espaço duplo para novas sentenças.
  - Espaço triplo para indentações.

Por exemplo, o seguinte código:
```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```
Deve tornar-se:
```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Notas

Antes de realizar um PR, por favor, certifique-se que suas mudanças estejam consistentes com as digitalizações!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
