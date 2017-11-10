# 阿波羅11號
[![NASA][1]][2]

*Available in: [English](README.md), [简体中文](README.zh_cn.md), [正體中文](README.zh_tw.md), [Português](README.pt_br.md), [Español](README.es.md), [한국어](README.ko_kr.md)*
 
原版的阿波羅11號電腦（Apollo 11 Guidance computer, AGC）用於駕駛艙（Comanche055）和登月艙（Luminary099）的原始碼，由 [Virtual AGC](http://www.ibiblio.org/apollo/) 及 [MIT Museum](http://web.mit.edu/museum/) 的工作人員數位化，目的在於建立一個存放阿波羅11號原始碼的程式庫。正因如此，若你在文件抄錄上或在查看 [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) 和 [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/) 這兩部分原始碼過程中遇到問題的話，我們歡迎你能提起一個 PR。當然，也包括那些我可能遺忘的文件。

## 編譯

如果對編譯原始碼有興趣的話，參考 [Virtual AGC](https://github.com/rburkey2005/virtualagc) 。

## 權限
```plain
版權：      公眾所有
文件名稱：  CONTRACT_AND_APPROVALS.agc
目的：      部份為 Colossus 2A 的原始碼，也稱為 Comanche 055
           阿波羅11號導引電腦（Apollo 11 Guidance computer, AGC）駕駛艙原始碼的一部份
組譯器：    yaYUL
連絡人：    Ron Burkey <info@sandroid.org>.
網站：      www.ibiblio.org/apollo.
修改紀錄:   2009-05-06 RSB  從文件圖片抄錄出來

這份原始碼是經過轉錄，或是從 MIT Museum 的數位化影像複印本整理出來的。
數位化是由 Paul Fjeld 完成，並經過 MIT Museum 的 Deborah Douglas 整理，在此衷心感激兩位。
這些影像（適當地減少了影像大小，以及降低了影像解析度）已經可以在網站 www.ibiblio.org/apollo 上觀看。
若你發現有字跡模糊的情形，請通過電子郵件 info@sandroid.org 聯繫我，以獲得高清影像。
事實上，這些影像是 Paul 建立的。

在這份複印本中，我們可以看到當中有這樣的一段注釋：

  由 NASA 的 Comanche 所編寫的 AGC 彙編修訂版 055
  2021113-051. 1969年4月1日 10:28

第一頁

#************************************************************************
#                                                                       *
#           該 AGC 程序也可能被稱作                                       *
#                                                                       *
#                                                                       *
#               COLOSSUS 2A                                             *
#                                                                       *
#                                                                       *
#   如 R-577 報告所指出的一樣，該程序主要用於阿波羅駕駛艙 CM。              *
#   此外，該程序是由 DSR 計劃 55-23870 所籌劃，並由美國                    *
#   國家航空航天局的太空總署通過合約 NAS 9-4065 所資助。                    *
#   該合約是由美國實驗儀器公司、麻省理工學院、劍橋大學以及                   *
#   馬斯共同簽定。                                                       *
#                                                                       *
#************************************************************************


提交者：  MARGARET H. HAMILTON        日期：   1969年3月28日
  M.H.HAMILTON, COLOSSUS 編程負責人
  阿波羅導引及導航功能

核准人：   DANIEL J. LICKLY           日期：   1969年3月28日
  D.J.LICKLY, 負責人, 任務程序開發
  阿波羅導引及導航功能程序

核准人：   FRED H. MARTIN             日期：   1969年3月28日
  FRED H. MARTIN, COLOSSUS 產品經理
  阿波羅導引及導航功能程序

核准人：   NORMAN E. SEARS            日期：   1969年3月28日
  N.E. SEARS, 負責人, 任務開發
  阿波羅導引及導航功能程序

核准人：   RICHARD H. BATTIN          日期：   1969年3月28日
  R.H. BATTIN, 負責人, 任務開發
  阿波羅導引及導航功能程序

核准人：   DAVID G. HOAG              日期：   1969年3月28日
  D.G. HOAG, 負責人
  阿波羅導引及導航功能程序

核准人：   RALPH R. RAGAN             日期：   1969年3月28日
  R.R. RAGAN, 副負責人
  美國實驗儀器公司
```

[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
