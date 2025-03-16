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

Dieses Repository beinhaltet den originalen Quellcode des Apollo 11 Navigationscomputers (kurz AGC) für das Kommandomodul (Comanche055) und die Mondlandefähre (Luminary099). Digitalisiert wurde der Code durch das [Virtual AGC][3] Projekt und das [MIT Museum][4]. Ziel dieses Projektes ist es, den originalen Apollo 11 Source Code an einem zentralen Ort zu sammeln. Daher sind PRs, die Diskrepanzen zwischen den Transkripten in diesem Repository und den originalen Scans des Source Codes von [Luminary 099][5] und [Comanche 055][6] beheben, gern gesehen. Das Gleiche gilt für irgendwelche Dateien, die ich verpasst haben könnte.

## Mitmachen

Bitte lies [CONTRIBUTING.de.md][7], bevor du einen Pull Request öffnest.

## Kompilieren

Wenn du den originalen Quellcode gern selbst kompilieren möchtest, wirf am besten einen Blick auf das [Virtual AGC][8] Projekt.

## Attribution

&nbsp;          | &nbsp;
:-------------- | :-----
Urheberrecht    | Gemeingut
Comanche055     | Teil des Quellcodes für Colossus 2A, den Apollo Navigationscomputer (AGC) des Kommandomoduls (CM) für Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099     | Teil des Quellcodes für Luminary 1A, den Apollo Navigationscomputer (AGC) der Mondlandefähre (LM) für Apollo 11<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembler       | yaYUL
Kontakt         | Ron Burkey <info@sandroid.org>
Webseite        | www.ibiblio.org/apollo
Digitalisierung | Der vorliegende Quellcode wurde von digitalisierten Bildern einer physischen Kopie des MIT Museums transkribiert oder anderweitig adaptiert. Die Digitalisierung wurde durchgeführt von Paul Fjeld nach Vorbereitung von Deborah Douglas vom MIT Museum. Vielen Dank an beide.

### Vertrag und Genehmigungen

*Abgeleitet aus [CONTRACT_AND_APPROVALS.agc]*

Dieses AGC Programm wird gleichfalls als Colossus 2A bezeichnet.

Dieses Programm ist für die Benutzung im CM vorgesehen, wie in Report `R577` spezifiziert. Das Programm wurde vorbereitet im Rahmen des DSR Projekts `55-23870`, gesponsert vom Zentrum für bemannte Raumfahrt der National Aeronautics and Space Administration durch Vertrag `NAS 9-4065`, geschlossen mit dem Instrumentation Laboratory des Massachusetts Institute of Technology, Cambridge, Mass.

Eingereicht von      | Position | Datum
:------------------- | :------- | :----
Margaret H. Hamilton | Leitende Colossus Programmiererin<br>Apollo Steuerung und Navigation | 28. März 1969

Genehmigt von     | Position | Datum
:---------------- | :------- | :----
Daniel J. Lickly  | Direktor, Mission Program Development<br>Apollo Steuerungs- und Navigationsprogramm | 28. März 1969
Fred H. Martin    | Colossus Projektmanager<br>Apollo Steuerungs- und Navigationsprogramm | 28. März 1969
Norman E. Sears   | Direktor, Mission Development<br>Apollo Steuerungs- und Navigationsprogramm | 28. März 1969
Richard H. Battin | Direktor, Mission Development<br>Apollo Steuerungs- und Navigationsprogramm | 28. März 1969
David G. Hoag     | Direktor<br>Apollo Steuerungs- und Navigationsprogramm | 28. März 1969
Ralph R. Ragan    | Stellvertretender Direktor<br>Instrumentation Laboratory | 28. März 1969

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.de.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
