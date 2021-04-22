# Contributing

🎌
[Čeština][CZ],
[Deutsch][DE],
**English**,
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Lietuvių][LT],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Türkçe][TR],
[Ελληνικά][GR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]
[Tiếng Việt][VI],

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
[PL]:CONTRIBUTING.pl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md
[VI]:CONTRIBUTING.vi.md

Mã nguồn trong kho lưu trữ này đã được số hóa thủ công từ các bản in trên giấy, do đó lỗi chính tả và các sai lệch khác đã vô tình được đưa vào. Mã nguồn sẽ được sửa đổi để phù hợp với các bản in được quét sau đây:

- [Bản in AGC cho Comanche][8]
- [Bản in AGC cho Luminary][9]

## Mở rộng tiện ích 

GitHub có hỗ trợ cú pháp cho ngôn ngữ hợp ngữ AGC được tích hợp sẵn. Rất tiếc, trình soạn thảo mã nguồn của bạn sẽ không thể, tuy nhiên, có các phần mở rộng ngôn ngữ AGC cung cấp đánh dấu cú pháp cho các trình chỉnh sửa sau:

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

† Hỗ trợ định dạng tự động

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

## Định dạng 

**Lưu ý:** GitHub và các tiện ích mở rộng được đánh dấu ở trên sẽ đảm bảo bạn đang sử dụng đúng định dạng tự động.

- Sử dụng thụt đầu dòng
- Sử dụng chiều rộng là 8
- Cắt bỏ khoảng trắng ở cuối

## Tôi nên kiểm tra cái gì?

Bất kỳ sự khác biệt nào giữa bản quét và mã nguồn trong kho lưu trữ này.

### Mục bình luận

Các bình luận trong mã nguồn đã phiên âm **PHẢI** khớp **chính xác** với các bản quét.

Các vấn đề phổ biến bạn nên chú ý trong khi soát lỗi bao gồm, nhưng không giới hạn ở:

#### Lỗi đánh máy

Ở một số chỗ, các nhà phát triển ban đầu đã mắc lỗi đánh máy trong khi viết bình luận. Một số trong số này đã được sửa chữa một cách nhầm lẫn trong quá trình số hóa ban đầu, tuy nhiên quá trình số hóa cũng đã đưa ra các lỗi đánh máy không có trong bản quét.

Ví dụ: nếu các nhận xét được số hóa chứa `SPACECRAFT`, nhưng `SPAECRAFT` đã được in trong bản quét, thì số hóa **PHẢI** được sửa thành `SPAECRAFT` (bỏ kí tự `C`).

Tương tự như vậy, nếu một từ có lỗi đánh máy trong số hóa nhưng lại được viết đúng chính tả trong bản quét thì lỗi chính tả ** PHẢI ** sẽ được sửa.

#### Khoảng cách

Khoảng cách giữa hai ký tự trong bình luận **NÊN** khớp với bản quét. Trong hầu hết các trường hợp (xem thảo luận trong [#316][10]), đây là:

- Dấu cách đơn cho các từ mới.
- Gấp đôi khoảng cách cho các câu mới.
- Gấp ba khoảng cách cho các thụt dòng.

Not all pages in the scans follow this generalisation, if the scans only have a single space instead of a double space, use a single space.

### Ngắt dòng

- Ngắt dòng *với* `R0000` trong cột 1 phải khớp chính xác với các lần quét.
- Ngắt dòng *với**__out__* `R0000` trong cột 1 chỉ được chứa 1 hoặc 2 dòng trống liên tiếp.
   - Nếu có nhiều hơn 2 dấu ngắt dòng trống thì ngắt dòng thừa.
     - Các dòng có `R0000` trong cột 1 không được tính vào dòng này.
   - Trong các nguồn ảnh, chúng được tạo bởi một chữ số không được in trong cột 8. A 2 buộc phải có khoảng trắng kép (dòng trống đơn) và a 3 buộc phải có ba khoảng trắng (dòng trống kép). Giá trị 4-8 đã được xác định nhưng không bao giờ được sử dụng. Đọc thêm về nó trong [#159] [7]

Ví dụ như sau:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Nên trở thành:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Lưu ý

Trước khi bạn thực hiện một Pull Request, hãy đảm bảo rằng các thay đổi của bạn phù hợp với bản quét!
