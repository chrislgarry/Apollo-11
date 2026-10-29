# Сприяння

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

Вихідний код у цьому репозиторії було оцифровано вручну з паперових роздруківок, тому помилки та інші розбіжності були внесені випадково. Код необхідно змінити, щоб узгодити його з такими сканованими роздруківками:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

На наступному веб-сайті можна легко переглянути відскановані роздруківки Comanche і Luminary: https://28gpc.csb.app/

## Корисні розширення

GitHub має вбудовану підтримку синтаксису для мови асемблера AGC. На жаль, ваш редактор коду цього не зробить, однак існують розширення мови AGC, які забезпечують підсвічування синтаксису для таких редакторів:

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

† Підтримує автоматичне форматування

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

## Форматування

**Примітка.** GitHub і розширення, позначені вище, забезпечать автоматичне використання правильного форматування.

- Використовуйте відступ через TAB.
- Використовуйте TAB шириною (width) у 8.
- Видаляйте кінцеві пробіли.

## Що мені перевірити?

Будь-які розбіжності між сканами та вихідним кодом у цьому репозиторії.

### Коментарі

Коментарі в транскрибованому коді **МАЮТЬ** **точно** збігатися зі сканами.

Поширені проблеми, на які слід звернути увагу під час перевірки, вміщують, але не обмежуються таким:

#### Друкарські помилки

У деяких місцях оригінальні розробники допускали друкарські помилки під час написання коментарів. Деякі з них були помилково виправлені під час початкової оцифровки, однак оцифрування також допустило друкарські помилки, яких не було в сканованих документах.

Наприклад, якщо оцифровані коментарі містили `SPAECRAFT`, але `SPAECRAFT` було надруковано в сканах, тоді оцифрування **ПОТРІБНО** виправити на `SPAECRAFT` (відсутнє `C`).

Аналогічно якщо слово має друкарську помилку під час оцифрування, але написано правильно в сканах, тоді друкарську помилку **ПОТРІБНО** виправити.

#### Пробіли

Пробіли між двома символами в коментарях **ПОВИННІ** збігатися зі сканами. У більшості випадків (див. обговорення у [#316][10]), це:

- Одиничний пробіл для нових слів.
- Подвійний пробіл для нових речень.
- Потрійний пробіл для відступів.

Не всі скановані сторінки дотримуються цього узагальнення. Якщо скани мають лише один пробіл замість подвійного, використовуйте один пробіл.

### Розриви рядків

- Розриви рядків *з* `R0000` у стовпці 1 мають точно відповідати сканам.
- Розриви рядків *з**__out__* `R0000` у стовпці 1 мають містити лише 1 або 2 порожні рядки поспіль.
  - Якщо є більше 2 порожніх розривів рядків, видаліть додаткові розриви рядків.
    - Рядки з `R0000` у стовпці 1 не враховуються.
  - На вихідних зображеннях вони були створені недрукованою цифрою в стовпці 8. 2 примусово ставило подвійний пробіл (один порожній рядок), а 3 — потрійний пробіл (подвійний порожній рядок). Значення 4-8 були визначені, але ніколи не використовувалися. Докладніше про це в [#159][7].

Наприклад, наступне:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Має стати:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Примітка

Перш ніж робити PR, будь ласка, переконайтеся, що ваші зміни узгоджуються зі сканами!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
