<div dir="RTL">

# المساهمة

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

تمّت رقمنة الكود المصدر في هذا المستودع يدويًا من مطبوعات ورقية، لذلك طرأت أخطاء طباعية و&nbsp;تباينات أخرى سهوًا. يجب تعديل الكود ليتّفق مع المسوحات الضوئية للطبعات التالية:

- [طبعات AGC لكود Comanche][8]
- [طبعات AGC لكود Luminary][9]

The following website can be used to easily navigate around the scanned printouts for both Comanche and Luminary: https://28gpc.csb.app/

## ملحقات مفيدة

يحوي GitHub دعمًا للتلوين النحويِّ للغة التجميع AGC، و&nbsp;مع أنّ محررات الكود الشائعة لا تدعم ذلك، إلا أنه ثمّة ملحقات توفر الدعم للغة AGC لكلٍ مِنْ المحررات التالية:

- [Atom][Atom]۞
- [CodeBlocks][CodeBlocks]
- [Eclipse][Eclipse]
- [Kate][Kate]
- [ProgrammersNotepad][ProgrammersNotepad]
- [Sublime Text 3][Sublime Text]†
- [TextPad][TextPad]
- [Vim][Vim]
- [Visual Studio Code][VisualStudioCode]†
- [jEdit][jEdit]

۞ يدعم التنسيق الآلي

## التنسيق

**ملاحظة:** GitHub و&nbsp;الملحقات المذكورة أعلاه تضمن تلقائيًّا استخدام التنسيق الصحيح.

- استخدام علامة الجدولة للإزاحة أوّل السطر.
- تعيين طول علامة الجدولة بثمان مسافات.
- حذف المسافات اللاحقة للنصوص في السطر

## المطلوب التحقّق منه

التباينات بين المسوحات الضوئية و&nbsp;الكود المصدر في هذا المستودع، بما في ذلك:

### التعليقات

يجب أن تتطابق التعليقات في الكود المنسوخ مع ما في المسوحات حرفيًّا

من الجوانب التي ينبغي تحرّيها عند المراجعة:

#### الأخطاء الطباعية

في بعض المواضع وقع مطوّرو البرمجية في أخطاء طباعية أثناء إدخال نصوص التعليقات، و&nbsp;قد تم تصويب بعض تلك الأخطاء من باب الخطأ في أثناء إنجاز الرقمنة ابتداءً، كما طرأت أخطاء أثناء الرقمنة لم توجد في الأصل.

على سبيل المثال، إذا تضمّنت التعليقات في الكود في المستودع كلمة `SPACECRAFT` بينما وجدت في الموضع المقابل في المسوحات الضوئية `SPAECRAFT` فيجب تصويب النص إلى `SPAECRAFT` بإغفال حرف `C`

كذلك إذا وُجد في كلمة في الكود المٌرقمَن خطأ طباعي غير موجود في المسوحات الضوئية فيجب تصويبه.

#### المسافات

يجب أن تُطابِق المسافات بين الأحرف في الكود المُرقمَن نظيراتها في المسوحات الضوئية، و&nbsp;ذلك يكون على النحو التالي في أغلب الحالات (طالعوا النقاش في [#316][10]):

- مسافة واحدة بين الكلمات
- مسافتين بين العبارات
- ثلاث مسافات لإزاحة أوّل السطر

- لكن الصفحات في المسوحات الضوئية لا تتبع كلّها تلك القواعد، و&nbsp;عند التباين يجب اتّباع ما في المسوحات الضوئية.

### السطور

- السطور التي *تحوي* `R0000` في العمود 1 يجب أن تطابق الممسوحات حرفيًّا.
- السطور التي __لا__ *تحوي* `R0000` في العمود 1 يجب أن يسبقها سطر فارغ واحد أو اثنين متتاليين.
  - إذا وُجد أكثر من سطرين فارغين متتاليين يجب حذف الزائد منها.
    - لا تؤخذ الأسطر التي تحوي `R0000` في العمود 1 في الحسبان.
  - في المصدر، أنشئت تلك الفواصل بوضع رقم غير مطبوع في العمود 8. فوجود الرقم 2 في ذلك الموضع أوجد فراغًا مزدوجًا (سطر فارغ واحد) و&nbsp;وجود الرقم 3 أوجد فراغًا ثلاثيًّا (سطرين فارغين). القيم 4-8 كانت مُعرَّفة لكنها لم تستخدم قط. المزيد عن هذا في [#159][7]

على سبيل المثال:

<div dir="ltr">

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

</div>

يُصوّب إلى:
<div dir="ltr">

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

</div>

## ملاحظة

قبل وضع طلب الدمج، يُرجى التأكد من اتفاق تحريراتكم مع المسوحات!

</div>

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741

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
