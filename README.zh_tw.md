# 阿波羅 11 號
[![NASA][1]][2]

:crossed_flags:
[English][EN],
[Español][ES],
[Français][FR],
[Português][PT_BR],
正體中文,
[简体中文][ZH_CN],
[한국어][KO_KR]

[EN]:README.md
[ES]:README.es.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md

最初用於阿波羅 11 號的導航電腦 (Apollo 11 Guidance computer, AGC) 裡頭駕駛艙 (Comanche055) 和登月艙 (Luminary099) 的原始碼，由 [Virtual AGC](http://www.ibiblio.org/apollo/) 及 [MIT Museum](http://web.mit.edu/museum/) 的工作人員進行數位化，著眼於建立阿波羅 11 號原始碼的封存。正因如此，若您在文件抄錄上或於查看 [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) 和 [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/) 這兩部分原始碼過程中遇到問題的話，歡迎提交 pull request。當然，也包括那些我可能遺忘的文件。

## 編譯

若您對編譯原始碼有興趣，請見 [Virtual AGC](https://github.com/rburkey2005/virtualagc) 。

## 資訊和專案屬性
```plain
授權聲明：  公眾領域
檔案名稱：  CONTRACT_AND_APPROVALS.agc
目的：      部份為 Colossus 2A 的原始碼，也稱為 Comanche 055
            阿波羅 11 號導航電腦 (Apollo 11 Guidance computer, AGC) 駕駛艙原始碼的一部份
組譯器：    yaYUL
連絡人：    Ron Burkey <info@sandroid.org>.
網站：      www.ibiblio.org/apollo
編修紀錄:   2009-05-06 RSB  自文件圖片抄錄出來

這份原始碼整理自 MIT Museum 的數位化影像複印本。
數位化是由 Paul Fjeld 完成，並經過 MIT Museum 的 Deborah Douglas 整理，在此衷心感激兩位。
這些影像 (適當地減少影像尺寸，並縮減影像解析度) 已可在網站 www.ibiblio.org/apollo 上觀看。
若您發現有字跡模糊的情形，請透過電子郵件 info@sandroid.org 聯繫本人，以取得原本高解析度的影像。

在這份複印本中，我們可見其中出現以下注釋：

  由 NASA Comanche 開發的 AGC 程式修訂版 055
  2021113-051. 1969 年 4 月 1 日 10:28

第一頁

#************************************************************************
#                                                                       *
#           該 AGC 程式也可能被稱作                                     *
#                                                                       *
#                                                                       *
#               COLOSSUS 2A                                             *
#                                                                       *
#                                                                       *
#   如同 R-577 報告指出，該程式主要用於阿波羅駕駛艙。                   *
#   此外，該程式是由 DSR 計劃 55-23870 籌劃，並由美國                   *
#   國家航空航天局的太空總署經由合約 NAS 9-4065 資助。                  *
#   該合約是由美國實驗儀器公司、麻省理工學院、劍橋大學以及              *
#   MASS共同簽定。                                                      *
#                                                                       *
#************************************************************************


提交者：  MARGARET H. HAMILTON        日期：   1969 年 3 月 28 日
  M.H.HAMILTON, COLOSSUS 編程負責人
  阿波羅導引導航系統

核准人：   DANIEL J. LICKLY           日期：   1969 年3 月 28 日
  D.J.LICKLY, 負責人, 任務程式開發
  阿波羅導引導航程式

核准人：   FRED H. MARTIN             日期：   1969 年 3 月 28 日
  FRED H. MARTIN, COLOSSUS 專案經理
  阿波羅導引導航程式

核准人：   NORMAN E. SEARS            日期：   1969 年 3 月 28 日
  N.E. SEARS, 負責人, 任務開發
  阿波羅導引導航程式

核准人：   RICHARD H. BATTIN          日期：   1969 年 3 月 28 日
  R.H. BATTIN, 負責人, 任務開發
  阿波羅導引導航程式

核准人：   DAVID G. HOAG              日期：   1969 年 3 月 28 日
  D.G. HOAG, 負責人
  阿波羅導引導航程式

核准人：   RALPH R. RAGAN             日期：   1969 年 3 月 28 日
  R.R. RAGAN, 副負責人
  美國實驗儀器公司
```

[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
