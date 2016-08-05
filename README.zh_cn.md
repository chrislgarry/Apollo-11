阿波罗11号
=========

*Also available in: [English](README.md), [简体中文](README.zh_cn.md), [正體中文](README.zh_tw.md).*

这是起初，阿波罗11号指引计算机（AGC）用于驾驶舱（Comanche055）和登月舱（Luminary099）的源代码。其是由身处 [Virtual AGC](http://www.ibiblio.org/apollo/) 及 [MIT Museum](http://web.mit.edu/museum/) 的各位工作人员所抄录出来，目的在于建立一个用于存放阿波罗11号源码的项目。正因如此，若你在项目抄录或在查看 [Luminary 099](http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/) 和 [Comanche 055](http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/) 这两部分源代码的过程中，明确地遇到了问题的话，我们欢迎你能为之提起一个 PR。当然，也包括那些我可能会遗忘的文件。

## 编译

如果对编译该源码有兴趣的话，可以查阅一下 [Virtual AGC](https://github.com/rburkey2005/virtualagc) 的文档。

## 权限

     版权：    公众所有。
     文件名：  CONTRACT_AND_APPROVALS。
     目的：    开放 Colossus 2A、 AKA Comanche 055 中的部分源码。
              开放阿波罗11号指引计算机中用于驾驶舱（CM）的部分源码。
     汇编器：  yaYUL
     联系人：  Ron Burkey <info@sandroid.org>。
     网站：    www.ibiblio.org/apollo。
     模块历史纪录：   2009-05-06 RSB  从页面的图片抄录出来

     该源代码是通过 MIT Musuem 一个复印本中的图片所抄录出来，并以其他方式进行改编。
     图片的抄录是由 Paul Fjeld 所完成，而整理则由 Museum 的 Deborah Douglas 所进行。
     因此，在此衷心感激两位。这些图片（适当地减少了存储空间，以及降低了图像质量）已经
     可以在网站 www.ibiblio.org/apollo 上进行观看。如果由于部分原因需要到图片的合法使用，请
     通过邮箱 info@sandroid.org 联系我，以获取这些高质量图片的使用权。其实，这些图片实际上
     是属于 Paul 的.

     在这份复印本中，我们可以看到其中这样的一段注释：

        由 NASA 的 Comanche 所编写的 AGC 汇编修订版 055
        2021113-051.  10:28 1969年8月1日

     第一页

    #************************************************************************
    #                                                                       *
    #           该 AGC 程序可能也会被称作                                     *
    #                                                                       *
    #                                                                       *
    #               COLOSSUS 2A                                             *
    #                                                                       *
    #                                                                       *
    #   如 R-577 报告所指出的一样，该程序主要用于阿波罗驾驶舱 CM。              *
    #   此外，该程序是由 DSR 项目 55-23870 所准备，并由（美国）                *
    #   国家航空与航天局的载人航天中心通过合同 NAS 9-4065 所资助。              *
    #   该合同是由美国实验仪器公司、麻省理工学院及剑桥马斯共同签定。             *
    #                                                                       *
    #************************************************************************


    提交者：  MARGARET H. HAMILTON        日期：   1969年3月28日
        M.H.HAMILTON, COLOSSUS 编程负责人
        阿波罗的指导及导航功能

    核准人：   DANIEL J. LICKLY           日期：   1969年3月28日
        D.J.LICKLY, 负责人、任务程序开发
        阿波罗的指导及导航功能程序

    核准人：   FRED H. MARTIN             日期：   1969年3月28日
        FRED H. MARTIN, COLOSSUS 产品经理
        阿波罗的指导及导航功能程序

    核准人：   NORMAN E. SEARS            日期：   1969年3月28日
        N.E. SEARS, 负责人、任务程序开发
        阿波罗的指导及导航功能程序

    核准人：   RICHARD H. BATTIN          日期：   1969年3月28日
        R.H. BATTIN, 负责人、任务开发
        阿波罗的指导及导航功能程序

    核准人：   DAVID G. HOAG              日期：   1969年3月28日
        D.G. HOAG, 负责人
        阿波罗的指导及导航功能程序

    核准人：   RALPH R. RAGAN             日期：   1969年3月28日
        R.R. RAGAN, 副负责人
        美国实验仪器公司
