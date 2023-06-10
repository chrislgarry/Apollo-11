# 贡献

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
[Türkçe][TR],
[Ελληνικά][GR],
[العربية][AR],
[日本語][JA],
[正體中文][ZH_TW],
**简体中文**,
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

本仓库中的源代码来自对纸质打印输出文件的手动数字化，所以会不小心引入一些拼写错误和其他差异。这些代码应该被修正以使其与原纸质打印输出文件内容保持一致：

- [阿波罗 11 号制导计算机（AGC）中指令模块打印输出][8]
- [阿波罗 11 号制导计算机（AGC）中登月模块打印输出][9]

## 有用的扩展

GitHub 内建支持 AGC 汇编语言语法。不幸的是，你的代码编辑器并不支持，但有些
AGC 语言的扩展插件可以为下列的编辑器提供语法高亮显示支持：

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

† 符号表示支持自动格式化

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

**注意**：GitHub 和上面列出的3个扩展插件会确保您自动使用正确的格式。

- 使用 TAB 进行缩进
- TAB 宽度为 8 个字符
- 去掉末尾的空格

## 我该怎么检查？

原始的扫描打印输出文件与此存储库中的源码之间存在的差异，包括：

### 注释

- 转录代码中的注释应与扫描件完全匹配
  - 这可能涉及故意创建的拼写错误或删除/添加整个评论

### 换行

- 第 1 列 *带* `R0000` 的换行符应与扫描件完全匹配。
- 第 1 列 *不**__带__* `R0000` 的换行符应该只包含 1 个或 2 个连续空行。
  - 如果有超过 2 个空行换行，请删除额外的空白行换行。
  - 第 1 列中带 `R0000` 的行不计入此。
  - 在原始图片中，这些是由一个未打印的数字分 8 列创建。A 2 处强制两个空格（单个空白行），A 3 处强制三个空格（双空行）。值 4-8 被定义但从未使用过。可在 [#159][7] 查看细节

例如以下内容：

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

应该改为：

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### 空格

- 字符串中两个字符之间的空格应遵循以下约定 (可在 [#316][10] 查阅相关讨论)：
  - 新单词另起一个空格.
  - 新句另起两个空格.
  - 缩进占三个空格.

例如以下内容：

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

应该改为：

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## 谨记

在提交 PR 之前，请确保您的更改与原始的扫描件打印输出一致！

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
