<div dir="RTL">

# المساهمة

🎌
[Čeština][CZ]،
[Deutsch][DE]،
[English][EN]،
[Español][ES]،
[Français][FR]،
[Italiano][IT]،
[Kurdi][KU]،
[Lietuvių][LI]،
[Nederlands][NL]،
[Norsk][NO]،
[Português][PT_BR]،
[Türkçe][TR]،
[Ελληνικά][GR]،
**العربية**،
[日本語][JA]،
[正體中文][ZH_TW]،
[简体中文][ZH_CN]،
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CZ]:CONTRIBUTING.cz.md
[DE]:CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[GR]:CONTRIBUTING.gr.md
[IT]:CONTRIBUTING.it.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[LT]:CONTRIBUTING.lt.md
[NL]:CONTRIBUTING.nl.md
[NO]:CONTRIBUTING.no.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

تم ترقيم الكود المصدري في هذا المستودع يدويًا من مطبوعات ورقية، لذلك تم إدخال أخطاء مطبعية وما شابه بطريق الخطأ. يجب تعديل الكود ليكون متوافقًا مع المطبوعات الممسوحة:

- [مطبوعات AGC لComanche][8]
- [مطبوعات AGC لLuminary][9]

## ملحقات مفيدة

يحتوي GitHub على دعم للغة تجميع AGC المضمنة. للاسف، لن يدعمه محرر الكود الخاص بك، ولكن هناك ملحقات توفر دعم للغة AGC من المحررين التاليين:

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

† يدعم التنسيق الآلي

## التنسيق

**ملاحظة:**  سيضمن GitHub والإضافات الثلاثة المذكورة أعلاه أنك تستخدم التنسيق الصحيح تلقائيًا.

- استخدام علامة التبويب للمسافة البادئة.
- استخدم العرض 8 لعلامة التبويب
- تقليم الفراغات التابعة

## عما أفحص؟

أي تباينات بين الممسوحات والكود المصدري في هذا المستودع، بما في ذلك:

### التعليقات

- يجب أن تتطابق التعليقات في الكود المنسوخ مع الممسوحات بالضبط
  - قد يتضمن ذلك إنشاء خطأ مطبعي متعمد أو إزالة أو إضافة تعليق بأكمله.

### فواصل الأسطر

- على فواصل الأسطر التي *تحتوي على* `R0000` في العمود 1 التطابق مع الممسوحات بالضبط.
- على فواصل الأسطر التي __لا__ *تحتوي على* `R0000` في العمود 1 أن تتكون من سطر فارغ واحد أو إثنين متتاليين.
  - إذا تتالى أكثر من سطرين فارغين، جرد الأسطر الإضافية.
    - لا يتم إعتبار الأسطر التي تحتوي على `R0000` في العمود 1 في هذا.
  - في المصدر، تم إنشاء هذه الفواصل السطرية من خلال رقم غير مطبوع في العمود 8. وجود الرقم 2 هناك فرض فراغ مزدوج (سطر فارغ واحد) ووجود الرقم 3 فرض مساحة ثلاثية (سطران فارغان). تم تعريف القيم 4-8 ولكن لم تستخدم قط. اقرأ المزيد عنها في [#159][7]

مثلاً، ما يلي:
</div>

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

<div dir="RTL">
يجب أن يصبح:
</div>

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

<div dir="RTL">

### الفراغات

- يجب أن تحترم الفراغات بين الحروف في سلاسل الاصطلاحية على حسب العرف التالي (راجع النقاش في [#316][10]):
  - فراغ واحد للكلمات الجديدة.
  - فراغان للجمل الجديدة.
  - ثلاث فراغات للمسافات البادئة.

مثلاً، ما يلي:
</div>

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE، GRAULT،
	GARPLY، WALDO.
```

<div dir="RTL">
يجب أن يصبح:
</div>

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE، GRAULT،
	   GARPLY، WALDO.
```

<div dir="RTL">

## ملاحظة

قبل فتح طلبات السحب، يرجى التأكد من أن تغييراتك تتفق مع الممسوحات!

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
