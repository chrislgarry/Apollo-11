# 貢獻

:crossed_flags:
[English][EN],
[Español][ES],
[Français][FR],
[Nederlands][NL],
[Português][PT_BR],
[Türkçe][TR],
[Deutsch][DE],
[العربية][AR],
**正體中文**,
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[NL]:CONTRIBUTING.nl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md
[DE]:CONTRIBUTING.de.md

這個倉庫的原始碼，是透過列印出的紙本輸出，予以手動數位化。所以可能會不小心出現錯字和其他出入。程式碼應當修正到與紙本掃描的輸出一致：

* [AGC printouts for Comanche][8]
* [AGC printouts for Luminary][9]

## 實用套件

GitHub 內建 AGC 組語語法支援，但你的編輯器本身並沒有。不過，某些編輯器有提供 AGC 語法突顯的語言套件：
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

† 支援自動格式化

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

## 格式化
**注意：**GitHub 與上面列出的套件，會自動確保您使用正確的格式。

- 用 tab 縮排
- tab 寬度為 8
- 去除底部空白

## 如何檢查？
掃描與倉庫原始碼的差異會包括：

### 註解
- 抄寫的註解要和掃描一模一樣。
  - 這包括刻意的拼寫錯誤、或是刪除/增加整個註解。

### 換行
- 在第一列（column 1）*有著* `R0000` 的換行，要和掃描一模一樣。
- 在第一列*沒有* `R0000` 的換行，在一行（row）只能留有一到兩個空白行。
  - 如果空白行超過兩個，請刪去額外的空白行。
    - 如果在第一列有 `R0000` 的話，就不要這麼做。
  - 在原始圖片中，這些都是由第八列（column 8）的未列印數字所產生。2 強制用兩個（也就是單一空白行）、3 強制用三個（也就是雙空行）、4-8 有定義，但從未使用過。請參閱 [#159][7] 的詳細訊息。

例如以下程式：
```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```
要改寫成：
```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### 空白
- 在字元內，兩個單字之間的空白必須遵照以下慣例（請參閱 [#316][10] 的討論）：
  - 單字間空一格。
  - 句子間空兩格。
  - 縮排間空三格。

例如以下程式：
```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```
要改寫成：
```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## 注意

在發 PR 前，請確保您的修改與掃描輸出一模一樣！

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[3]:https://github.com/wopian/agc-assembly
[4]:https://github.com/jimlawton/AGC-Assembly
[5]:https://github.com/Alhadis/language-agc
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
