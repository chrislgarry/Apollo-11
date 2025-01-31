# アポロ11号

[![NASA][1]][2]
[![SWH]][SWH_URL]
[![Comanche]][ComancheMilestone]
[![Luminary]][LuminaryMilestone]

🌐
[Azerbaijani][AZ],
[bahasa Indonesia][ID],
[Català][CA],
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Galego][GL],
[Italiano][IT],
[Kurdî][KU],
[Lietuvių][LT],
[Mongolian][MN],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Română][RO],
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Беларуская мова][BE],
[Русский][RU],
[Українська][UK],
[العربية][AR],
[فارسی][FA],
[नेपाली भाषा][NE]
[हिंदी][HI_IN],
[অসমীয়া][AS_IN],
[বাংলা][BD_BN],
[မြန်မာ][MM],
[한국어][KO_KR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN]

[AR]:README.ar.md
[AS_IN]:README.as_in.md
[AZ]:README.az.md
[BD_BN]:README.bd_bn.md
[BE]:README.be.md
[CA]:README.ca.md
[CZ]:README.cz.md
[DA]:README.da.md
[DE]:README.de.md
[EN]:../README.md
[ES]:README.es.md
[FA]:README.fa.md
[FR]:README.fr.md
[GL]:README.gl.md
[GR]:README.gr.md
[HI_IN]:README.hi_in.md
[ID]:README.id.md
[IT]:README.it.md
[JA]:README.ja.md
[KO_KR]:README.ko_kr.md
[KU]:README.ku.md
[LT]:README.lt.md
[MM]:README.mm.md
[MN]:README.mn.md
[NE]:README.ne.md
[NL]:README.nl.md
[NO]:README.no.md
[PL]:README.pl.md
[PT_BR]:README.pt_br.md
[RO]:README.ro.md
[RU]:README.ru.md
[TR]:README.tr.md
[UK]:README.uk.md
[VI]:README.vi.md
[ZH_CN]:README.zh_cn.md
[ZH_TW]:README.zh_tw.md

司令船・機械船(Comanche055)および月着陸船(Luminary099)用のオリジナルのアポロ11号誘導コンピュータ(AGC)のソースコード。[Virtual AGC][3] と [MIT Museum][4] によってデジタル化された。このリポジトリは、オリジナルのアポロ11号のソースコードを完全に再現して保管することを目的としています。そのため、このリポジトリと [Luminary 099][5] および [Comanche 055][6] の間に発見された問題や見落としがある可能性のあるファイルに対してのPRは歓迎します。

## 貢献

プルリクエストを開く前に [CONTRIBUTING.ja.md][7] をお読みください。

## コンパイル

もしコンパイルをご希望の場合 [Virtual AGC][8] を確認してください。

## 権限

&nbsp;      | &nbsp;
:---------- | :-----
著作権       | パブリックドメイン
Comanche055 | アポロ11号用の司令船・機械船用のアポロ11号誘導コンピュータ、Colossus 2Aのソースコードの一部<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099 | アポロ11号用の月着陸船用のアポロ11号誘導コンピュータ、Luminary 1Aのソースコードの一部<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
アセンブラ    | yaYUL
コンタクト    | Ron Burkey <info@sandroid.org>
ウェブサイト  | www.ibiblio.org/apollo
デジタル化    | このソースコードは、MIT Museumからハードコピーをデジタル化、移したものです。 デジタル化は Paul Fjeld によって行われ、 Deborah Douglas がまとめました。

### 契約と承認

*[CONTRACT_AND_APPROVALS.agc] から派生*

本アポロ誘導コンピュータプログラムは、コロッサス 2Aから参照されます。

このプログラムは、`R-577` に指定された司令船モジュールに使用されます。このプロジェクトは、DSR project `55-23870` の元で準備され、NASAの有人宇宙船センターがMIT機械研究所との `NAS 9-4065` 契約により始まりました。

Submitted by         | Role | Date
:------------------- | :--- | :---
Margaret H. Hamilton | コロッサス プログラミングリーダー<br>アポロ誘導と航海 | 1969年3月28日

Approved by       | Role | Date
:---------------- | :--- | :---
Daniel J. Lickly  | ディレクター、ミッションプログラムの開発<br>アポロ誘導と航法プログラム | 1969年3月28日
Fred H. Martin    | コロッサス プロジェクトマネージャー<br>アポロ誘導と航法プログラム | 1969年3月28日
Norman E. Sears   | ディレクター、ミッションプログラムの開発<br>アポロ誘導と航法プログラム | 1969年3月28日
Richard H. Battin | ディレクター、ミッションプログラムの開発<br>アポロ誘導と航法プログラム | 1969年3月28日
David G. Hoag     | ディレクター<br>アポロ誘導と航法プログラム | 1969年3月28日
Ralph R. Ragan    | 副ディレクター<br>機械研究所 | 1969年3月28日

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.ja.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
