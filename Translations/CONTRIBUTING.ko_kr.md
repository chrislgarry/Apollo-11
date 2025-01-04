# 기여하기

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

이 저장소의 소스 코드는 종이 인쇄물에서 수동으로 디지털화되었으므로 실수로 오타 및 기타 불일치가 발생했습니다. 코드는 다음 스캔된 인쇄물과 일관되도록 수정해야 합니다.

- [Comanche용 AGC 인쇄물][8]
- [Luminary용 AGC 인쇄물][9]

다음 웹사이트를 사용하면 Comanche와 Luminary의 스캔된 인쇄물을 쉽게 탐색할 수 있습니다. https://28gpc.csb.app/

## 유용한 확장기능들

GitHub은 내장 에디터에서 AGC문법을 지원합니다. 그러나 불행하게도 대개 사용되는 에디터들은 그렇지 않습니다. 하지만 확장 기능이 지원되는 에디터들이 있습니다.

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

† 자동 포매팅 지원

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

## 포매팅

**참고:** GitHub 내장 에디터와 위의 세 확장기능들은 이 문단에서 정의하는 포매팅을 자동으로 수행해 줍니다.

- 탭 들여쓰기를 사용하십시오.
- 들여쓰기 크기는 8입니다.
- 뒤에 불필요하게 붙는 공백 문자는 제거하시기 바랍니다.

## 확인해야 할 것들

### 주석

- 옮겨진 코드의 주석들은 스캔본과 일치해야 합니다.
  - 의도적인 오타나 전체 주석을 삭제/추가하는 것과 관계가 있을 수 있습니다.

### 개행

- 1열에서 `R0000`로 개행되는 경우 스캔본과 일치해야 합니다.
- 1열에서 `R0000`*__없이__*  개행되는 경우 1~2개의 빈 줄이 있어야 합니다.
  - 만약 3개 이상의 개행이 있는 경우 초과분을 제거하십시오.
    - 1열에 `R0000`를 포함하는 경우 빈 줄로 치지 않습니다.
  - 소스 이미지는 8열의 인쇄되지 않은 숫자로 생성되었습니다.
  두 부분에 두 개의 공백과(빈 줄 한 개), 세 부분에 3개의 공백이(빈 줄 두 개) 강제됩니다.
  4-8의 값들은 정의되었지만 사용되지 않습니다. 자세한 내용은 [#159][7]를 참고하십시오.

예를 들어 이 코드는:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

이렇게 고쳐야 합니다:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
