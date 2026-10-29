# Hướng dẫn đóng góp

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

Mã nguồn trong repo này được số hóa một cách thủ công từ các bản in giấy, do đó lỗi đánh máy và các lỗi khác đã vô tình được đưa vào. Code sẽ được sửa để phù hợp với các bản in được quét sau:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Trang web sau đây có thể dễ dàng duyệt qua với các bản in được quét của cả Comanche và Luminary: https://28gpc.csb.app/

## Các tiện ích mở rộng

GitHub tích hợp sẵn hỗ trợ cú pháp cho ngôn ngữ lắp assembly AGC. Thật không may, trình soạn thảo của bạn sẽ không hỗ trợ, tuy nhiên có các tiện ích mở rộng ngôn ngữ AGC cung cấp tính năng highlight cú pháp cho các trình soạn thảo sau:

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

† Hỗ trợ tự động định dạng (Auto formatting)

[Atom]: https://github.com/Alhadis/language-agc
[CodeBlocks]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/CodeBlocks
[Eclipse]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Eclipse
[Kate]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Kate
[ProgrammersNotepad]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/ProgrammersNotepad
[Sublime Text]: https://github.com/jimlawton/AGC-Assembly
[TextPad]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/TextPad
[Vim]: https://github.com/wsdjeg/vim-assembly
[VisualStudioCode]: https://github.com/wopian/agc-assembly
[jEdit]: https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/jEdit

## Định dạng

**Lưu ý:** GitHub và các tiện ích mở rộng được đánh dấu ở trên sẽ đảm bảo bạn đang sử dụng định dạng chính xác một cách tự động.

- Sử dụng canh chĩnh tab
- Sử dụng chiều rộng tab là 8
- Xóa khoảng trắng cuối

## Tôi cần kiểm tra cái gì?

Bất kỳ sự khác biệt nào giữa các bản quét và mã nguồn trong repo này.

### Comments

Các comment trong code **PHẢI** khớp **chính xác** với bản quét.

Các vấn đề phổ biến mà bạn nên chú ý khi kiểm tra, nhưng không giới hạn, bao gồm:

#### Lỗi đánh máy

Ở một số nơi, các nhà phát triển ban đầu đã mắc lỗi đánh máy khi viết comment. Một số lỗi này đã được sửa nhầm trong quá trình số hóa ban đầu, tuy nhiên quá trình số hóa cũng đã đưa vào các lỗi đánh máy không có trong bản quét.

Ví dụ, nếu các bình luận được số hóa có chứa `SPACECRAFT`, nhưng `SPAECRAFT` được in trong bản quét, thì bản số hóa **PHẢI** được sửa thành `SPAECRAFT` (thiếu `C`).

Tương tự như vậy, nếu một từ có lỗi đánh máy trong bản số hóa nhưng được viết đúng chính tả trong bản quét thì **PHẢI** được sửa lỗi đánh máy đó.

#### Khoảng trắng

Khoảng trắng giữa hai ký tự trong comment **NÊN** khớp với bản quét. Trong hầu hết các trường hợp (xem phần thảo luận trong [#316][10]), như sau:

- 1 khoảng trắng cho các từ mới.
- 2 khoảng trắng cho các câu mới.
- 3 khoảng trắng cho các thụt lề.

Không phải tất cả các trang trong bản quét đều tuân theo quy tắc trên, nếu bản quét chỉ có một khoảng trắng  thay vì hai khoảng trắng, hãy sử dụng một khoảng trắng.

### Ngắt dòng

- Ngắt dòng _có_ `R0000` trong cột 1 phải khớp chính xác với bản quét.
- Ngắt dòng _\*\***không** có_ `R0000` trong cột 1 chỉ nên chứa 1 hoặc 2 dòng trống liên tiếp.
  - Nếu có nhiều hơn 2 ngắt dòng trống, hãy xóa các ngắt dòng thừa.
    - Các dòng có `R0000` ở cột 1 sẽ không được tính vào đây.
  - Trong các hình ảnh nguồn, Những thứ trên được tạo ra bởi một chữ số không được in vào trong cột 8. Một số 2 ở đó buộc phải có hai khoảng trắng (một dòng trống) và một số 3 buộc phải có ba khoảng trắng (hai dòng trống). Các giá trị 4-8 được xác định nhưng không bao giờ được sử dụng. Đọc thêm tại [#159][7]

Hãy xem ví dụ dưới đây:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Nên là:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Ghi chú

Trước khi bạn tạo PR, hãy chắc chắn rằng những thay đổi của bạn phù hợp và ổn định với những scan sau:

[0]: https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]: http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]: http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]: https://github.com/wopian/agc-assembly#user-settings
[7]: https://github.com/chrislgarry/Apollo-11/issues/159
[8]: http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]: http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]: https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
