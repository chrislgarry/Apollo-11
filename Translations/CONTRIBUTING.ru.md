# Участие в проекте

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

Исходный код в этом репозитории был оцифрован вручную с бумажных распечаток. При этом в него случайно попали опечатки и другие несоответствия. Исправления должны приводить код в соответствие со следующими сканами исходных распечаток:

- [Распечатки программы Comanche для AGC][8]
- [Распечатки программы Luminary для AGC][9]

Для удобной навигации по сканам Comanche и Luminary можно использовать сайт https://28gpc.csb.app/

## Полезные расширения

GitHub поддерживает подсветку синтаксиса ассемблера AGC по умолчанию. В редакторах кода такая поддержка обычно отсутствует, но её можно добавить с помощью расширений для следующих редакторов:

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

† Поддерживают автоматическое форматирование

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

**Примечание:** GitHub и расширения, отмеченные выше символом †, автоматически обеспечивают правильное форматирование.

- Используйте табуляцию для отступов
- Установите ширину табуляции в 8 символов
- Удаляйте пробелы в конце строк

## Что проверять?

Любые расхождения между сканами и исходным кодом в этом репозитории.

### Комментарии

Комментарии в оцифрованном коде **ДОЛЖНЫ** совпадать со сканами **точно**.

Ниже перечислены распространённые проблемы, на которые следует обратить внимание при проверке. Этот список не является исчерпывающим.

#### Опечатки

В комментариях исходных разработчиков встречаются опечатки. Некоторые из них ошибочно исправили при первоначальной оцифровке. При этом появились и новые опечатки, которых нет на сканах.

Например, если в оцифрованном комментарии написано `SPACECRAFT`, а на скане — `SPAECRAFT`, в оцифрованном коде **НУЖНО** восстановить `SPAECRAFT` (без буквы `C`).

Аналогично, если при оцифровке появилась опечатка, а на скане слово написано правильно, эту опечатку **НУЖНО** исправить.

#### Пробелы

Пробелы между символами в комментариях **СЛЕДУЕТ** воспроизводить по сканам. В большинстве случаев (см. обсуждение в [#316][10]) используются:

- Один пробел между словами.
- Два пробела между предложениями.
- Три пробела для отступа.

Это правило соблюдается не на всех страницах: если на скане один пробел вместо двух, используйте один.

### Разрывы строк

- Разрывы строк *с* `R0000` в столбце 1 следует воспроизводить в точном соответствии со сканами.
- Между строками *без* `R0000` в столбце 1 следует оставлять только 1 или 2 пустые строки подряд.
  - Если подряд идут более 2 пустых строк, удалите лишние.
    - Строки с `R0000` в столбце 1 при этом не учитываются.
  - В исходных распечатках эти интервалы задавались непечатаемой цифрой в столбце 8. Цифра 2 задавала двойной интервал (одна пустая строка), а 3 — тройной (две пустые строки). Значения 4–8 были определены, но никогда не использовались. Подробнее — в [#159][7].

Например:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Должно получиться:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Примечание

Перед созданием PR убедитесь, что ваши изменения соответствуют сканам!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
