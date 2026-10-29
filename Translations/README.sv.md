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
[Suomi][FI],
[Svenska][SV],
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Беларуская мова][BE],
[Русский][RU],
[Українська][UK],
[العربية][AR],
[فارسی][FA],
[Հայերեն][HY],
[नेपाली भाषा][NE],
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
[FI]:README.fi.md
[FR]:README.fr.md
[GL]:README.gl.md
[GR]:README.gr.md
[HI_IN]:README.hi_in.md
[HY]:README.hy.md
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
[SV]:README.sv.md
[TR]:README.tr.md
[UK]:README.uk.md
[VI]:README.vi.md
[ZH_CN]:README.zh_cn.md
[ZH_TW]:README.zh_tw.md

Original Apollo 11 guidance computer (AGC) källkod för Command Module (Comanche055) och Lunar Module (Luminary099). Digitaliserad av folket på [Virtual AGC][3] och [MIT Museum][4]. Målet är att vara ett kodarkiv för den ursprungliga Apollo 11-källkoden. Som sådan är PR:er välkomna för alla problem som identifieras mellan transkriptionerna i detta arkiv och de ursprungliga källsökningarna för [Luminary 099][5] och [Comanche 055][6], såväl bortglömda filer.

## Bidra

Läs [CONTRIBUTING.sv.md][7] innan du öppnar en "pull request".

## Kompilering

Om du är intresserad av att kompilera den ursprungliga källkoden, kolla
ut [Virtuell AGC][8].

## Tillskrivning

&nbsp;         | &nbsp;
:------------- | :-----
Upphovsrätt      | Allmän egendom
Comanche055    | En del av källkoden för Colossus 2A, Command Modules (CM) Apollo Guidance Computer (AGC) för Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099    | En del av källkoden för Luminary 1A, Lunar Modules (LM) Apollo Guidance Computer (AGC) för Apollo 11<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assemblerare      | yaYUL
Kontakt        | Ron Burkey <info@sandroid.org>
Webbplats        | www.ibiblio.org/apollo
Digitalisering | Denna källkod har transkriberats eller på annat sätt anpassats från digitaliserade bilder av en papperskopia från MIT Museum. Digitaliseringen utfördes av Paul Fjeld och arrangerades av Deborah Douglas från museet. Stort tack till båda.

### Kontrakt och godkännanden

*Härrör från [CONTRACT_AND_APPROVALS.agc]*

Detta AGC-program kommer också att kallas Colossus 2A.

Det här programmet är avsett för användning i kommandomodulen som specificeras i rapporten `R-577`. Detta program förbereddes under DSR-projektet `55-23870`, sponsrat av Manned Spacecraft Center vid National Aeronautics and Space Administration genom kontraktet `NAS 9-4065` med Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass.

Inskickad av         | Roll | Datum
:------------------- | :--- | :---
Margaret H. Hamilton | Colossus Programming Leader<br>Apollo Guidance and Navigation | 28 Mar 69

Godkänd av       | Roll | Datum
:---------------- | :--- | :---
Daniel J. Lickly  | Director, Mission Program Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Fred H. Martin    | Colossus Project Manager<br>Apollo Guidance and Navigation Program | 28 Mar 69
Norman E. Sears   | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Richard H. Battin | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
David G. Hoag     | Director<br>Apollo Guidance and Navigation Program | 28 Mar 69
Ralph R. Ragan    | Deputy Director<br>Instrumentation Laboratory | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.sv.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
