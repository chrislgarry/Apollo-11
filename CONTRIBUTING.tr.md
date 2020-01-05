# Katkıda Bulunma

:crossed_flags:
[English][EN],
[Português][PT_BR],
**Türkçe**,
[العربية][AR],
[한국어][KO_KR]

[EN]:CONTRIBUTING.md
[AR]:CONTRIBUTING.ar.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[TR]:CONTRIBUTING.tr.md

Bu repodaki kaynak kodu kağıt çıktılarından elle dijitalleştirilmiştir. Yani yazım hataları ve diğer tutarsızlıklar yanlışlıkla yapılmıştır. Kod, aşağıdaki taratılmış çıktılar ile tutarlı olması için düzenlenmelidir:

* [AGC printouts for Comanche][8]
* [AGC printouts for Luminary][9]

## Yararlı Eklentiler

Github, AGC assembly dili için sentaks desteği sağlıyor ama sizin kodu düzenlemek için kullandığınız editör sağlamayacaktır. Aşağıdaki editörler AGC dili sentaks vurgulaması için eklentiler sunmaktadır:
- [Atom][Atom] (supports auto formatting)
- [CodeBlocks][CodeBlocks]
- [Eclipse][Eclipse]
- [Kate][Kate]
- [ProgrammersNotepad][ProgrammersNotepad]
- [Sublime Text 3][Sublime Text] (supports auto formatting)
- [TextPad][TextPad]
- [Vim][Vim]
- [Visual Studio Code][VisualStudioCode] (supports auto formatting)
- [jEdit][jEdit]

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
Taratılmış çıktılar ve bu repodaki kaynak kodu arasındaki aşağıdakileri kapsayan herhangi bir tutarsızlığı:

### Yorumlar
- Uyarlanan koddaki yorum satırları, çıktılar ile tamamen eşleşmelidir
  - Bu bilerek yazım hatası koymayı veya bir yorumun tamamını kaldırıp/eklemeyi içerebilir.

### Satır sonları
- Birinci sütunu `R0000` *ile* biten satır sonları çıktılar ile tamamen eşleşmelidir.
- Birinci sütunu `R0000` *__olmadan__* biten satır sonları arka arkaya sadece 1 ya da 2 boş satır içermelidir.
  - Eğer 2'den fazla satır sonu varsa ekstra olan satır sonlarını kaldırın.
    - Birinci sütunu `R0000` *ile* biten satırlar bu kural dahilinde değildir.
  - Kaynak resimlerde bunlar 8. satırındaki basamak bastırılmamış halde oluşturulmuştur. Bir 2 kullanımı iki boşluğu (tek boş satırı) temsil ederken, bir 3 kullanımı üç boşluğu (iki boş satırı) ifade eder. 4 ve 8 tanımlanmıştır fakat hiç kullanılmamıştır. Bunun hakkında daha fazla okumak için: [#159][7].

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

### Boşluklar
- Bir string'deki iki karakterin arasındaki boşluklar şu kurallara uymalıdır ([#316][10]'daki tartışmayı inceleyin):
  - Yeni kelimeler arası tek boşluk.
  - Yeni cümleler arası iki boşluk.
  - Yeni girintiler için üç boşluk.

Örneğin bu:
```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```
Buna dönüşmeli:
```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
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
