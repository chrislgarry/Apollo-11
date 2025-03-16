# Apollo-11

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

사령선 모듈 (Comanche055) 및 달 착륙선 모듈 (Luminary099)을 위한 아폴로 11호 유도 컴퓨터(AGC)의 소스코드입니다. [Virtual AGC][3] 및 [MIT Museum][4]에 의해 디지털화되었습니다. 이 레포지토리는 아폴로 11호의 원본 소스코드를 완벽하게 재현하여 보관하는 것을 목표로 합니다. 따라서 원본과 본 디지털본 간에 발견된 모든 이슈, 또한 빠뜨렸을 듯한 파일들에 대해 PR을 환영합니다.

## 기여하기

PR을 열기 전에 [CONTRIBUTING.ko_kr.md][7] 을 읽어보시기 바랍니다.

## 컴파일

만약 컴파일을 원하신다면 [Virtual AGC][8] 을 확인하여 보십시오.

## 권한

&nbsp;      | &nbsp;
:---------- | :-----
저작권       | 퍼블릭 도메인
Comanche055 | Part of the source code for Colossus 2A, the Command Module's (CM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099 | Part of the source code for Luminary 1A, the Lunar Module's (LM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
어셈블러     | yaYUL
연락처       | Ron Burkey <info@sandroid.org>
웹사이트     | www.ibiblio.org/apollo
디지털화     | 이 소스코드는 MIT Museum에서 하드 카피본을 디지털화, 옮긴 것입니다. 디지털화는 Paul Fjeld에 의해 이루어졌으며, Deborah Douglas가 정리하였습니다. 두 분께 깊은 감사를 표합니다.

### Contract and Approvals

*[CONTRACT_AND_APPROVALS.agc] 에서 파생 됨*

본 아폴로 유도 컴퓨터 프로그램은 다음에 의해 참조됩니다 : 콜로서스 2A

이 프로그램은 R-577에 명세된 사령선 모듈에 사용됩니다. DSR 프로젝트 55-23870에 의해 준비되었으며, NASA 유인 우주선 센터가 MIT 기계 연구소와의 NAS 9-4065 계약에 의해 스폰싱 하였습니다.

제출됨                | Role | 일시
:------------------- | :--- | :--
Margaret H. Hamilton | 콜로서스 프로그래밍 리더<br>아폴로 유도 및 항해 | 1969년 3월 28일

승인됨             | Role | 일시
:---------------- | :--- | :--
Daniel J. Lickly  | 감독, 임무 프로그램 개발<br>아폴로 유도 및 항법 프로그램 | 1969년 3월 28일
Fred H. Martin    | 콜로서스 프로젝트 매니저<br>아폴로 유도 및 항법 프로그램 | 1969년 3월 28일
Norman E. Sears   | 감독, 임무 개발<br>아폴로 유도 및 항법 프로그램 | 1969년 3월 28일
Richard H. Battin | 감독, 임무 개발<br>아폴로 유도 및 항법 프로그램 | 1969년 3월 28일
David G. Hoag     | 감독<br>아폴로 유도 및 항법 프로그램 | 1969년 3월 28일
Ralph R. Ragan    | 부감독<br>기계 연구소 | 1969년 3월 28일

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.ko_kr.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
