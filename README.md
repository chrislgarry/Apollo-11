Apollo-11 阿波羅11號
=========

起初阿波羅11號的指引計算機 (AGC) 源代碼. 其包含有該計算機的命令模塊 (Comanche055) 及登月艙模塊 (Luminary099). 它是由身處 [Virtual AGC](http://www.ibiblio.org/apollo/) 及 [MIT Museum](http://web.mit.edu/museum/) 的各位人員所抄錄出來的. 目的在於建立一個項目, 用於存放阿波羅11號的源代碼. 正因如此, 若你在項目抄錄或為 [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) 和 [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/) 所進行的源代碼查看過程中, 遇到了任何問題, 我們都歡迎你能為之提起一個 PR. 當然, 也包括那些我可能會遺忘的文件.

Original Apollo 11 guidance computer (AGC) source code for Command Module (Comanche055) and Lunar Module (Luminary099). Digitized by the folks at [Virtual AGC](http://www.ibiblio.org/apollo/) and [MIT Museum](http://web.mit.edu/museum/). The goal is to be a repo for the original Apollo 11 source code. As such, PRs are welcome for any issues identified between the transcriptions in this repository and the original source scans for [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) and [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/), as well as any files I may have missed.

## Compilation 編撰

如果對編撰該源碼有興趣的話, 可以查閱一下 [Virtual AGC](https://github.com/rburkey2005/virtualagc) 的文檔.

If you are interested in compiling the original source code, check out [Virtual AGC](https://github.com/rburkey2005/virtualagc).

## Attribution 權限

     版權:    公眾所有.
     文件名:  CONTRACT_AND_APPROVALS.
     目的:    開放 Colossus 2A, AKA Comanche 055 中的部分源碼.
              開放阿波羅11號指引計算機中命令模塊 (CM) 的部分源碼.
     彙編器:  yaYUL
     聯繫人:  Ron Burkey <info@sandroid.org>.
     網站:    www.ibiblio.org/apollo.
     模塊歷史記錄:   2009-05-06 RSB  從頁面的圖片抄錄出來

     該源代碼是通過 MIT Musuem 一個複印本中的圖片所抄錄出來, 並以其他方式進行改編.
     圖片的抄錄是由 Paul Fjeld 所完成, 而由 Museum 的 Deborah Douglas 進行整理.
     因此, 在此衷心感激兩位. 這些圖片 (適當的減少了存儲空間, 以及降低了圖像質量) 已經
     可以在網站 www.ibiblio.org/apollo 觀看到. 如果你由於部分原因需要合法的圖片, 請
     通過郵箱 info@sandroid.org 聯繫我, 以獲取這些高質量圖片的使用權. 這些圖片實際上
     是屬於 Paul 的.

     在這份複印本中, 我們可以看到其中這樣的一段注釋:

        由 NASA 的 Comanche 所編寫的 AGC 彙編修訂版 055
        2021113-051.  10:28 1969年8月1日

     第一頁

    #************************************************************************
    #                                                                       *
    #           該 AGC 程序可能也會被稱作                                     *
    #                                                                       *
    #                                                                       *
    #               COLOSSUS 2A                                             *
    #                                                                       *
    #                                                                       *
    #   如 R-577 報告所指出的一樣, 該程序主要用於 CM.                          *
    #   此外, 该程序是由 DSR 項目 55-23870 所準備, 並由 (美國)                 *
    #   國家航空與航天局的載人航天中心通過合同 NAS 9-4065 所資助.               *
    #   該合同是由美國實驗儀器公司, 麻省理工學院, 劍橋馬斯共同簽定.              *
    #                                                                       *
    #************************************************************************


    SUBMITTED:  MARGARET H. HAMILTON        DATE:   28 MAR 69
        M.H.HAMILTON, COLOSSUS PROGRAMMING LEADER
        APOLLO GUIDANCE AND NAVIGATION

    APPROVED:   DANIEL J. LICKLY        DATE:   28 MAR 69
        D.J.LICKLY, DIRECTOR, MISSION PROGRAM DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   FRED H. MARTIN          DATE:   28 MAR 69
        FRED H. MARTIN, COLOSSUS PROJECT MANAGER
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   NORMAN E. SEARS         DATE:   28 MAR 69
        N.E. SEARS, DIRECTOR, MISSION DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   RICHARD H. BATTIN       DATE:   28 MAR 69
        R.H. BATTIN, DIRECTOR, MISSION DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   DAVID G. HOAG           DATE:   28 MAR 69
        D.G. HOAG, DIRECTOR
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   RALPH R. RAGAN          DATE:   28 MAR 69
        R.R. RAGAN, DEPUTY DIRECTOR
        INSTRUMENTATION LABORATORY

<br />    

     Copyright: Public domain.
     Filename:  CONTRACT_AND_APPROVALS.agc
     Purpose:   Part of the source code for Colossus 2A, AKA Comanche 055.
                It is part of the source code for the Command Module's (CM)
                Apollo Guidance Computer (AGC), for Apollo 11.
     Assembler: yaYUL
     Contact:   Ron Burkey <info@sandroid.org>.
     Website:   www.ibiblio.org/apollo.
     Mod history:   2009-05-06 RSB  Transcribed from page images.

     This source code has been transcribed or otherwise adapted from digitized
     images of a hardcopy from the MIT Museum.  The digitization was performed
     by Paul Fjeld, and arranged for by Deborah Douglas of the Museum.  Many
     thanks to both.  The images (with suitable reduction in storage size and
     consequent reduction in image quality as well) are available online at
     www.ibiblio.org/apollo.  If for some reason you find that the images are
     illegible, contact me at info@sandroid.org about getting access to the
     (much) higher-quality images which Paul actually created.

     Notations on the hardcopy document read, in part:

        Assemble revision 055 of AGC program Comanche by NASA
        2021113-051.  10:28 APR. 1, 1969  

     Page 1

    #************************************************************************
    #                                                                       *
    #       THIS AGC PROGRAM SHALL ALSO BE REFERRED TO AS:                  *
    #                                                                       *
    #                                                                       *
    #               COLOSSUS 2A                                             *
    #                                                                       *
    #                                                                       *
    #   THIS PROGRAM IS INTENDED FOR USE IN THE CM AS SPECIFIED             *
    #   IN REPORT R-577.  THIS PROGRAM WAS PREPARED UNDER DSR               *
    #   PROJECT 55-23870, SPONSORED BY THE MANNED SPACECRAFT                *
    #   CENTER OF THE NATIONAL AERONAUTICS AND SPACE                        *
    #   ADMINISTRATION THROUGH CONTRACT NAS 9-4065 WITH THE                 *
    #   INSTRUMENTATION LABORATORY, MASSACHUSETTS INSTITUTE OF              *
    #   TECHNOLOGY, CAMBRIDGE, MASS.                                        *
    #                                                                       *
    #************************************************************************


    SUBMITTED:  MARGARET H. HAMILTON        DATE:   28 MAR 69
        M.H.HAMILTON, COLOSSUS PROGRAMMING LEADER
        APOLLO GUIDANCE AND NAVIGATION

    APPROVED:   DANIEL J. LICKLY        DATE:   28 MAR 69
        D.J.LICKLY, DIRECTOR, MISSION PROGRAM DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   FRED H. MARTIN          DATE:   28 MAR 69
        FRED H. MARTIN, COLOSSUS PROJECT MANAGER
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   NORMAN E. SEARS         DATE:   28 MAR 69
        N.E. SEARS, DIRECTOR, MISSION DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   RICHARD H. BATTIN       DATE:   28 MAR 69
        R.H. BATTIN, DIRECTOR, MISSION DEVELOPMENT
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   DAVID G. HOAG           DATE:   28 MAR 69
        D.G. HOAG, DIRECTOR
        APOLLO GUIDANCE AND NAVIGATION PROGRAM

    APPROVED:   RALPH R. RAGAN          DATE:   28 MAR 69
        R.R. RAGAN, DEPUTY DIRECTOR
        INSTRUMENTATION LABORATORY
