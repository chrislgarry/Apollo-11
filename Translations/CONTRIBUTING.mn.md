# Хувь нэмэр оруулж байна

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

Энэ репозиторийн эх код нь цаасан хэвлэлээс гараар дижитал хэлбэрт шилжүүлсэн тул зарим нэг бичгийн алдаа болон бусад зөрчил санаандгүйгээр орсон байж магадгүй. Доорх сканнердсан хэвлэмэлтэй тааруулахын тулд кодыг өөрчлөх шаардлагатай:

- [Comanche-ийн AGC хэвлэлүүд][8]
- [Luminary-ийн AGC хэвлэлүүд][9]

Дараах вэбсайтыг Comanche болон Luminary-ийн аль алиных нь сканнердсан хэвлэмэл материалуудаар хялбархан гүйлгэх боломжтой: https://28gpc.csb.app/

## Туслах өргөтгөлүүд

GitHub-д AGC угсрах хэлний синтаксийн дэмжлэг байдаг. Харамсалтай нь код засварлагчид байхгүй боловч дараах засварлагчид AGC хэлний өргөтгөлийг синтаксийн тодруулгыг дэмждэг:

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

† Автоформатлахыг дэмждэг

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

## Формат

**Анхаар:** GitHub болон дээрх өргөтгөлүүдийн тусламжтайгаар зөв формат автоматаар ашиглагдана.

- Таб ашиглан мөрийг хойшлуулна
- Табын өргөн нь 8 байна
- Мөрийн төгсгөлд байгаа хоосон зайг устгана

## Хэрхэн шалгах вэ?

Скан болон энэ репозиторид байгаа эх кодын хооронд зөрүү байгаа эсэхийг шалгана.

### Тайлбар

Текст болгон хувиргасан кодын тайлбарууд скантай **яг** таарч **байх ёстой**.

Хяналтын явцад анхаарах нийтлэг асуудлууд нь дараах байж болно, гэхдээ үүгээр хязгаарлагдахгүй:

#### Алдаа

Зарим газарт анхны хөгжүүлэгчид тайлбар бичихдээ алдаа гаргасан байдаг. Эдгээрийн зарим нь анхны дижиталчлалын үед засагдсан боловч дижиталчлалаас болж сканд байгаагүй алдаа гарсан тохиолдлууд ч бий.

Жишээлбэл, дижиталчлагдсан тайлбарт `SPACECRAFT` гэсэн үг байгаа боловч сканд `SPAECRAFT` гэж хэвлэгдсэн бол дижиталчлал нь `SPAECRAFT` гэж засагдах **ёстой** (`C` үсэг дутуу байна).

Үүнтэй адил, үгэнд дижиталчлалын алдаа байгаа боловч сканд зөв бичигдсэн бол алдааг засах **ёстой**.

### Зай

- Тайлбар доторх хоёр үсгийн хоорондох зай нь скантай **таарч байх ёстой**. Ихэнх тохиолдолд (дэлгэрэнгүйг [#316][10] хэлэлцүүлгээс үзнэ үү), дараах дүрмийг дагах хэрэгтэй:
  - Шинэ үгэнд нэг зай.
  - Шинэ өгүүлбэрт хоёр зай.
  - Хойшлуулахад гурван зай.

Сканы бүх хуудсууд энэ ерөнхий дүрмийг дагадаггүй. Хэрэв сканд хоёр зай биш нэг зай байгаа бол нэг зайг ашиглана уу.

### Мөр шилжүүлэлт

- `R0000`-тэй мөр шилжүүлэлт нь скантай яг таарч байх ёстой.
- `R0000`-тэй *биш* мөр шилжүүлэлт нь зөвхөн нэг эсвэл хоёр хоосон мөрийг агуулсан байх ёстой.
  - Хоосон мөр хоёр буюу түүнээс олон байвал илүүдэл мөрийг устгана.
    - `R0000`-тэй мөрүүд үүнд тооцогдохгүй.
  - Эх зургуудад эдгээр нь 8-р баганад хэвлэгдээгүй тоогоор үүсгэгдсэн. 2 нь давхар зай (нэг хоосон мөр), 3 нь гурвалсан зай (хоёр хоосон мөр) үүсгэдэг. 4-8 утгууд тодорхойлогдсон боловч ашиглагдаагүй. Дэлгэрэнгүйг [#159][7] үзнэ үү.

Жишээлбэл, дараах байдлаар:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Энэ болох ёстой:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Анхаар

PR үүсгэхээс өмнө өөрчлөлтүүд скантай таарч байгаа эсэхийг шалгана уу!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
