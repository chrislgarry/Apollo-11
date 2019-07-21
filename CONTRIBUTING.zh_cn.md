# Contributing

:crossed_flags:
[English][EN],
[العربية][AR],
[한국어][KO_KR]

中文

[EN]:CONTRIBUTING.md
[AR]:CONTRIBUTING.ar.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md

本仓库中的源码是通过对纸质打印输出进行手动数字化而来，所以会不小心引入了一些拼写错误和其他差异。应修改代码以使其与原始的扫描打印输出文件保持一致：

* [阿波罗 11 号制导计算机（AGC）中指令模块打印输出][8]
* [阿波罗 11 号制导计算机（AGC）中登月模块打印输出][9]

## 有用的扩展

GitHub内建有对AGC汇编语言的语法支持。不幸的是，你的代码编辑器不会，但有些
AGC的语言扩展插件可以为下面的编辑器提供语法高亮显示支持

- [Atom][5]
- [Sublime Text 3][4]
- [Visual Studio Code][3]

## 格式化
**注意**：GitHub和上面列出的3个扩展插件会确保您自动使用正确的格式。

- 使用TAB进行缩进
- TAB宽度为8个字符
- 去掉末尾的空格

## 我该怎么检查?
原始的扫描打印输出文件与此存储库中的源码之间存在的差异，包括：

### 注释
- 转录代码中的注释应与扫描完全匹配
  - 这可能涉及故意创建的拼写错误或删除/添加
    整个评论

### 换行
-  第1列带`R0000`的换行符应与扫描完全匹配。
- 第1列中带**__out__* `R0000` 的换行符应该只包含一个或2个连续空行。

  - 如果有超过2个空行换行，请删除额外的空白行换行。
  
    - 第1列中带`R0000`的行不计入此。
    
  - In the source images, these were created by an unprinted digit
    in column 8. A 2 there forced a double space (single blank line)
    and a 3 forced a triple space (double blank line). Values 4-8 were
    defined but never used. Read more about it in [#159][7]
  
    在原始图片中，这些是由一个未打印的数字分8列创建. A 2 处强制俩个空格（单个空白行），A 3 强制三个空格（双空行）。 值4-8是被定义但从未使用过。可在[#159][7]查看更多

例如以下内容:
```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```
应该改为:
```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### 空格
- 字符串中两个字符之间的空格应遵循以下约定 (可在 [#316][10]查阅相关讨论):
  - 新单词另起一个空格.
  - 新句另起俩个空格.
  - 缩进占三个空格.
  

例如以下内容:
```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```
应该改为:
```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## 谨记

在制作PR之前，请确保您的更改与原始的扫描打印输出一致！

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