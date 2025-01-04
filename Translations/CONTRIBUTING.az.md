# Töhfə

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

Bu repodakı mənbə kodu kağız çıxışlarından əl ilə rəqəmsallaşdırılıb. Belə ki, yanlışlıqla hərf səhvləri və digər uyğunsuzluqlar edilib. Kod aşağıdakı skan edilmiş çıxışa uyğun olmaq üçün redaktə edilməlidir:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

The following website can be used to easily navigate around the scanned printouts for both Comanche and Luminary: https://28gpc.csb.app/

## Faydalı Pluginlər

GitHub, AGC assembly dili için sentaks desteği sağlıyor ama sizin kodu düzenlemek için kullandığınız editör sağlamayacaktır. Aşağıdaki editörler AGC dili sentaks vurgulaması için eklentiler sunmaktadır:

GitHub, AGC assembly dilinin sintaksını dəstəkləyir amma sizin kodu redaktə etmək üçün istifadə edəcəyiniz IDE dəstəkləməyəcək. Aşağıdakı IDE-lər AGC dili üçün sintaks dəstəyi təqdim etməkdədir.

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

† Avtomatik formatlaşdırmanı dəstəkləyirlər

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

## Formatlaşdırma

**Not:** GitHub və yuxarıdakı 3 plagin avtomatik olaraq düzgün formatlaşdırmadan istifadə etməyə imkan verəcək.

- Girintiləməni TAB ilə buraxın.
- 8 TAB genişliyi istifadə edin.
- Davamındakı boşluqları silin.

## Nəyi kontrol edim?

Skan edilmiş çıxışlar və bu repodakı mənbə kodu arasında hər hansı uyğunsuzluq.

### Şərhlər

Uyğunlaşdırılmış koddakı şərh sətirləri çıxışlarla **dəqiq** **uyğunlaşmalıdır**.

Düzəltmək üçün axtarmaq lazım olan bəzi əsas problemlər bunlardır:

#### Yazım Xətaları

Bəzi yerlərdə orijinal tərtibatçılar şərhləri yazarkən hərf səhvlərinə yol verdiblər. Bunlardan bəziləri rəqəmsallaşdırma prosesinin əvvəlində səhvən düzəldildi, lakin rəqəmsallaşdırma prosesi skan edilmiş çıxışda olmayan yeni yazı xətaları da yol açdı.

Məsələn, rəqəmsal şərhlərdə `SPACECRAFT` sözü varsa, lakin skan edilmiş çaplarda `SPAECRAFT` oxunursa, rəqəmsal şərh `SPAECRAFT` olaraq **düzəldilməlidir** (`C` yoxdur).

Eyni şəkildə, rəqəmsal şərhlərdə yazı xətası varsa, lakin skan edilmiş çaplar düzgün yazılıbsa, yazı xətası **düzəldilməlidir**.

### Boşluqlar

Şərhlər arasındakı iki simvol arasındakı boşluqlar skan edilmiş çıxışa **uyğun** olmalıdır. Əksər hallarda ([#316][10]-da müzakirəyə baxın) boşluqlar aşağıdakılardır:

- Yeni sözlər arasında tək boşluq.
- Yeni cümlələr arasında iki boşluq.
- Yeni girintilər üçün üç boşluq.

Skan edilmiş çaplardakı bütün səhifələr bu ümumiləşdirmələrə uyğun gəlmir. Çıxışda iki boşluq əvəzinə boşluq varsa, bir boşluqdan istifadə edin.

### Sətir sonları

- İlk sütunu "R0000"  *ilə* bitən sətir sonları çıxışa tam uyğun gəlməlidir.
- Birinci sütunda `R0000` *__olmadan__* ilə bitən sətir sonunda yalnız 1 və ya 2 boş sətir olmalıdır.
  - Əgər 2-dən çox sətir sonu varsa artıq olanları silin.
    - Birinci sütunu `R0000` *ilə* bitən sətirlər bu qanun daxilində deyillər.
  - Mənbə şəkillərində onlar 8-ci sətirdə basdırılmamış rəqəmlə yaradılmışdır. 2 iki boşluğu (bir boş sətir), 3 isə üç boşluğu (iki boş sətir) təmsil edir. 4 və 8 müəyyən edilmişdir, lakin heç vaxt istifadə edilməmişdir. Bu barədə daha çox oxumaq üçün: [#159][7]

Məsələn bu:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Buna çevrilməlidir:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Not

PR-dən əvvəl, zəhmət olmasa, dəyişikliklərinizin nəticələrə uyğun olduğundan əmin olun!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
