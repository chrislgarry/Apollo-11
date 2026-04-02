# Сотрудничество

🌐
[Azerbaijani][AZ],
[bahasa Indonesia][ID],
[Basa Jawa][JV],
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
[简体中文][ZH_CN],
[Basa Jawa][JV]

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
[JV]:CONTRIBUTING.jv.md
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

Исходный код в этом репозитории был оцифрован вручную из бумажных распечаток, поэтому опечатки и другие несоответствия были введены случайно. Код должен быть изменён в соответствии со следующими отсканированными распечатками:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Следующий веб-сайт может быть использован для удобной навигации по отсканированным распечаткам для Comanche и Luminary: https://28gpc.csb.app/

## Полезные расширения

GitHub имеет встроенную поддержку синтаксиса для языка ассемблера AGC. К сожалению, ваш редактор кода этого не поддерживает, однако существуют расширения для языка AGC, которые обеспечивают выделение синтаксиса для следующих редакторов:

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

† Поддерживает автоматическое форматирование

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

## Форматирование

**Примечание:** GitHub и расширения, указанные выше, будут автоматически обеспечивать правильное форматирование.

- Используйте табуляцию для отступов
- Используйте ширину табуляции 8
- Удаляйте пробелы в конце строк

## Что я проверяю?

Любые несоответствия между отсканированными распечатками и исходным кодом в этом репозитории.

### Комментарии

Комментарии в переписанном коде **ДОЛЖНЫ** точно совпадать с отсканированными распечатками.

Распространённые проблемы, на которые следует обратить внимание при проверке, включают, но не ограничиваются:

#### Типографические ошибки

В некоторых местах первоначальные разработчики делали типографические ошибки при написании комментариев. Некоторые из них были ошибочно исправлены при первоначальной оцифровке, однако оцифровка также привела к типографическим ошибкам, которых не было в отсканированных распечатках.

Например, если оцифрованные комментарии содержали `SPACECRAFT`, но `SPAECRAFT` был напечатан в отсканированных распечатках, то оцифровка **ДОЛЖНА** быть исправлена на `SPAECRAFT` (отсутствует `C`).

Аналогично, если в оцифровке есть опечатка, но слово правильно написано в отсканированных распечатках, то опечатка **ДОЛЖНА** быть исправлена.

#### Пробелы

Пробелы между двумя символами в комментариях **СЛЕДУЕТ** совпадать с отсканированными распечатками. В большинстве случаев (см. обсуждение в [#316][10]), это:

- Один пробел для новых слов.
- Два пробела для новых предложений.
- Три пробела для отступов.

Не все страницы в отсканированных распечатках следуют этому обобщению; если в отсканированных распечатках только один пробел вместо двух, используйте один пробел.

### Разрывы строк

- Разрывы строк *с* `R0000` в столбце 1 следует совпадать с отсканированными распечатками.
- Разрывы строк *без* `R0000` в столбце 1 следует содержать только 1 или 2 пустые строки подряд.
  - Если есть более 2 пустых строк, удалите лишние разрывы строк.
    - Строки с `R0000` в столбце 1 не учитываются в этом.
  - На исходных изображениях они были созданы непечатаемой цифрой в столбце 8. Цифра 2 там вызывала двойной интервал (одна пустая строка), а 3 вызывала тройной интервал (две пустые строки). Значения 4-8 были определены, но никогда не использовались. Подробнее об этом можно прочитать в [#159][7]

Например, следующее:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Должно стать:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Примечание

Перед тем как создавать PR, убедитесь, что ваши изменения соответствуют отсканированным распечаткам!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
