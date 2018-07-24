# 阿波羅 11 號
[![NASA][1]][2]

:crossed_flags:
[English][EN],
[Español][ES],
[Français][FR],
[Português][PT_BR],
**正體中文**,
[简体中文][ZH_CN],
[한국어][KO_KR],
[हिंदी][HI_IN]

[EN]:README.md
[ES]:README.es.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md
[HI_IN]:README.hi_in.md

最初用於阿波羅 11 號的導航電腦 (Apollo 11 Guidance computer, AGC) 裡頭駕駛艙 (Comanche055) 和登月艙 (Luminary099) 的原始碼，由 [Virtual AGC](http://www.ibiblio.org/apollo/) 及 [MIT Museum](http://web.mit.edu/museum/) 的工作人員進行數位化，著眼於建立阿波羅 11 號原始碼的封存。正因如此，若您在文件抄錄上或於查看 [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) 和 [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/) 這兩部分原始碼過程中遇到問題的話，歡迎提交 pull request。當然，也包括那些我可能遺忘的文件。

## 編譯

若您對編譯原始碼有興趣，請見 [Virtual AGC](https://github.com/rburkey2005/virtualagc) 。

## 資訊和專案屬性

&nbsp;      | &nbsp;
:---------- | :-----
授權聲明     | Public domain
Comanche055 | Part of the source code for Colossus 2A, the Command Module's (CM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099 | Part of the source code for Luminary 1A, the Lunar Module's (LM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 001 of AGC program LYM99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
組譯器       | yaYUL
連絡人       | Ron Burkey <info@sandroid.org>
網站         | www.ibiblio.org/apollo
数字化       | 這份原始碼整理自 MIT Museum 的數位化影像複印本。數位化是由 Paul Fjeld 完成，並經過 MIT Museum 的 Deborah Douglas 整理，在此衷心感激兩位。

### Contract and Approvals
*Derived from [CONTRACT_AND_APPROVALS.agc]*

該 AGC 程式也可能被稱作: COLOSSUS 2A

如同 R-577 報告指出，該程式主要用於阿波羅駕駛艙。此外，該程式是由 DSR 計劃 55-23870 籌劃，並由美國 國家航空航天局的太空總署經由合約 NAS 9-4065 資助。該合約是由美國實驗儀器公司、麻省理工學院、劍橋大學以及MASS共同簽定。

提交者                 | Role | 日期
:-------------------- | :--- | :--
Margaret H. Hamilton  | Colossus 編程負責人<br>阿波羅導引導航系統 | 28 Mar 69

核准人              | Role | 日期
:----------------- | :--- | :--
Daniel J. Lickly   | 負責人, 任務程式開發<br>阿波羅導引導航程式 | 1969年 3月 28日
Fred H. Martin     | Colossus 專案經理<br>阿波羅導引導航程式 | 1969年 3月 28日
Norman E. Sears    | 負責人, 任務開發<br>阿波羅導引導航程式 | 1969年 3月 28日
Richard H. Battin  | 負責人, 任務開發<br>阿波羅導引導航程式 | 1969年 3月 28日
David G. Hoag      | 負責人<br>阿波羅導引導航程式 | 1969年 3月 28日
Ralph R. Ragan     | 副負責人<br>Instrumentation Laboratory | 1969年 3月 28日

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
