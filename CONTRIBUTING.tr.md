# Katkıda Bulunma

🎌
[Català][CA]،
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
**Türkçe**,
[Ελληνικά][GR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CA]:CONTRIBUTING.ca.md
[CZ]:CONTRIBUTING.cz.md
[DA]:CONTRIBUTING.da.md
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
[PL]:CONTRIBUTING.pl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Bu repodaki kaynak kodu kağıt çıktılarından elle dijitalleştirilmiştir. Yani yazım hataları ve diğer tutarsızlıklar yanlışlıkla yapılmıştır. Kod, aşağıdaki taratılmış çıktılar ile tutarlı olması için düzenlenmelidir:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Yararlı Eklentiler

GitHub, AGC assembly dili için sentaks desteği sağlıyor ama sizin kodu düzenlemek için kullandığınız editör sağlamayacaktır. Aşağıdaki editörler AGC dili sentaks vurgulaması için eklentiler sunmaktadır:

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

† Otomatik biçimlendirmeyi destekler

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

## Biçimlendirme

**Not:** GitHub ve yukarıdaki 3 eklenti otomatik olarak doğru biçimlendirmeyi kullanmanızı sağlayacaktır.

- Girintilemeyi tab ile yapın
- Tab genişliği olarak 8'i kullanın
- Devamındaki boşlukları silin

## Neyi kontrol edeyim?

Taratılmış çıktılar ve bu repodaki kaynak kodu arasındaki herhangi bir tutarsızlığı.

### Yorumlar

Uyarlanan koddaki yorum satırları, çıktılar ile **tamamen** **eşleşmelidir**.

Düzeltmek için bakmanız gereken bazı temel sorunlar şunlardır:

#### Yazım Hataları

Bazı yerlerde, orijinal geliştiriciler yorum yazarken yazım hataları yapmış. Bunların bazıları dijitalleştirme sürecinin başlarında yanlışlıkla düzeltildi fakat dijitalleştirme süreci taratılmış çıktılarda olmayan yeni yazım hatalarına da sebep oldu.

Örneğin, dijitalleştirilmiş yorumlar `SPACECRAFT` kelimesini içeriyorsa ama taratılmış çıktılarda `SPAECRAFT` yazıyorsa, dijitalleştirilmiş yorum `SPAECRAFT` (`C` eksik) olarak **düzeltilmelidir**.

Aynı şekilde, eğer dijitalleştirmiş yorumlarda bir yazım hatası varsa ama taratılmış çıktılarda doğru yazılmışsa, yazım hatası **düzeltilmelidir**.

### Boşluklar

Yorumlar arasındaki iki karakterin arasındaki boşluklar taratılmış çıktılar ile **uyuşmalıdır**. Bir çok durumda ([#316][10]'daki tartışmayı inceleyin) boşluklar şu şekildedir:

- Yeni kelimeler arası tek boşluk.
- Yeni cümleler arası iki boşluk.
- Yeni girintiler için üç boşluk.

Taratılmış çıktılarda bütün sayfalar bu genellemelere uymaz. Eğer çıktılarda iki boşluk yerine bir boşluk varsa, bir boşluk kullanın.

### Satır sonları

- Birinci sütunu `R0000` *ile* biten satır sonları çıktılar ile tamamen eşleşmelidir.
- Birinci sütunu `R0000` *__olmadan__* biten satır sonları arka arkaya sadece 1 ya da 2 boş satır içermelidir.
  - Eğer 2'den fazla satır sonu varsa ekstra olan satır sonlarını kaldırın.
    - Birinci sütunu `R0000` *ile* biten satırlar bu kural dahilinde değildir.
  - Kaynak resimlerde bunlar 8. satırındaki basamak bastırılmamış halde oluşturulmuştur. Bir 2 kullanımı iki boşluğu (tek boş satırı) temsil ederken, bir 3 kullanımı üç boşluğu (iki boş satırı) ifade eder. 4 ve 8 tanımlanmıştır fakat hiç kullanılmamıştır. Bunun hakkında daha fazla okumak için: [#159][7]

Örneğin bu:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Buna dönüşmeli:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Not

PR yapmadan önce lütfen yaptığınız değişikliklerin çıktılar ile tutarlı olduğundan emin olun!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
