# コントリビュート

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

このリポジトリのソースコードは紙の印刷物から手動でデジタル化された為、いくつかのタイプミスやその他の不一致が誤って導入されています。以下のスキャンしたプリントアウトと一致する様にコードを変更する必要があります:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

次の Web サイトを使用すると、Comanche と Luminary の両方のスキャンされたプリントアウトを簡単にナビゲートできます: https://28gpc.csb.app/

## 便利な拡張機能

GitHubには、組み込みのAGCアセンブリ言語の構文サポートがあります。残念ながらコードエディターにはありませんが、次のエディターにシンタックスハイライトをサポートするAGC言語拡張機能があります:

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

† オートフォーマットをサポート

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

## フォーマット

**注意:** GitHubと上記の拡張機能により、正しいフォーマットが自動的に使用されます。

- タブでインデントする
- タブ幅は8を使用する
- 末尾の空白を削除する

## 確認方法は?

スキャンとこのリポジトリ内のソースコードとの間の不一致がある場合。

### コメント

文字起こしされたコードのコメントはスキャンと**正確**に一致**しなければなりません**。

校正中に注意する必要がある一般的な問題には、次のものがありますが、これらに限定されません:

#### 誤植

いくつかの場所において、元の開発者がコメントを書いている中で誤植をしています。これらのいくつかは、最初のデジタル化の際に修正がされていますが、デジタル化によってスキャンに存在しなかった誤植も発生してしまいました。

例えば、デジタル化されたコメントに `SPACECRAFT` が含まれているが、 `SPAECRAFT` が印刷されたスキャンの場合、デジタル化は `SPAECRAFT` に修正**しなければなりません**(`C` が抜けている)。

同様に、単語のデジタル化にタイプミスがあるが、スキャンでスペルが正しい場合は、タイプミスを修正**しなければなりません**。

### スペース

- コメント内の2つの文字間のスペースは、スキャンと一致**すべきです**。多くの場合（[#316][10]のディスカッションを参照）、次の規則に従う必要があります:
  - 新しい単語の為の単一のスペース。
  - 新しい文章の為の2個のスペース。
  - インデントの為の3個のスペース。

スキャンのすべてのページがこの一般化に従っている訳ではありません。スキャンに2個のスペースではなく1個のスペースしかない場合、1個のスペースを使用して下さい。

### 改行

- 列1の `R0000` での改行は、スキャンと正確に一致する必要があります。
- 列1の `R0000` で *ない* 改行は、1行または2行の空白行のみを含める必要があります。
  - 空白の改行が2つ以上ある場合は、余分な改行を削除します。
    - 列1に `R0000` が含まれる行は、これにカウントされません。
  - ソース画像では、これらは列8の印刷されていない数字によって作成されました。2はダブルスペース（単一の空白行）を強制し、3はトリプルスペース（二重の空白行）を強制しました。値4-8は定義されていますが使用されていません。詳しくは[#159][7]をご覧ください。

例えば、次の通りです:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

これになるはずです:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## 注意

PRを作成する前に、変更がスキャンと一致していることを確認して下さい！

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
