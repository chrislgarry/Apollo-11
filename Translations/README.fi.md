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
[नेपाली भाषा][NE]
[हिंदी][HI_IN],
[অসমীয়া][AS_IN],
[বাংলা][BD_BN],
[မြန်မာ][MM],
[한국어][KO_KR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[മലയാളം][ML]

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
[ID]:README.id.md
[IT]:README.it.md
[JA]:README.ja.md
[KO_KR]:README.ko_kr.md
[KU]:README.ku.md
[LT]:README.lt.md
[ML]:README.ml.md
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

Alkuperäinen Apollo 11 guidance computer (AGC) lähdekoodi Command Module (Comanche055) ja Lunar Module (Luminary099) varten. Digitalisoitu [Virtual AGC][3] -proektin ja [MIT Museum][4] toimesta. Tavoitteena on toimia alkuperäisen Apollo 11 -lähdekoodin arkistona. Siksi pull requestit ovat tervetulleita, jos tässä repositoriossa olevien transkriptioiden ja alkuperäisten lähdekoodiskannausten ([Luminary 099][5] ja [Comanche 055][6]) välillä havaitaan eroavaisuuksia, tai jos jokin tiedosto on jäänyt puuttumaan.

## Osallistuminen

Ole hyvä ja lue [CONTRIBUTING.md][7] ennen pull requestin avaamista.

## Lähdekoodin kääntäminen

Jos olet kiinnostunut kääntämään alkuperäisen lähdekoodin, tutustu kohtaan [Virtual AGC][8].

## Lähdeviitteet

&nbsp;         | &nbsp;
:------------- | :-----
Tekijänoikeudet| Public domain
Comanche055    | Osa Colossus 2A -ohjelman lähdekoodista, joka toimi Apollo 11:n Command Modulen (CM) ohjaustietokoneessa Apollo Guidance Computer (AGC).<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099    | Osa Luminary 1A -ohjelman lähdekoodista, joka toimi Apollo 11:n kuumoduulin Lunar Module (LM) ohjaustietokoneessa Apollo Guidance Computer (AGC).<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembleri     | yaYUL
Yhteystiedot   | Ron Burkey <info@sandroid.org>
Sivusto        | www.ibiblio.org/apollo
Digitalisointi | Tämä lähdekoodi on kirjoitettu uudelleen tai muuten mukautettu MIT-museon hallussa olleen paperiversion digitoiduista kuvista. Digitalisoinnin suoritti Paul Fjeld, ja sen järjesti Deborah Douglas museolta. Suuret kiitokset heille molemmille.

### Sopimus ja Hyväksynnät

*Peräisin tiedostosta [CONTRACT_AND_APPROVALS.agc]*

Tätä AGC-ohjelmaa kutsutaan myös nimellä Colossus 2A.

Ohjelma on tarkoitettu käytettäväksi komentomoduulissa Command Module, kuten raportissa `R-577` on määritelty. Ohjelma on valmistettu DSR-projektin `55-23870` puitteissa, jota sponsoroi Manned Spacecraft Center of The National Aeronautics and Space Administration sopimuksen `NAS 9-4065` kautta yhteistyössä Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass. kanssa.

Lähettäjä            | Rooli | Päivämäärä
:------------------- | :--- | :---
Margaret H. Hamilton | Colossus Programming Leader<br>Apollo Guidance and Navigation | 28. Maaliskuuta 69

Hyväksyjät        | Rooli | Päivämäärä
:---------------- | :--- | :---
Daniel J. Lickly  | Director, Mission Program Development<br>Apollo Guidance and Navigation Program | 28. Maaliskuuta 69
Fred H. Martin    | Colossus Project Manager<br>Apollo Guidance and Navigation Program | 28. Maaliskuuta 69
Norman E. Sears   | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28. Maaliskuuta 69
Richard H. Battin | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28. Maaliskuuta 69
David G. Hoag     | Director<br>Apollo Guidance and Navigation Program | 28. Maaliskuuta 69
Ralph R. Ragan    | Deputy Director<br>Instrumentation Laboratory | 28. Maaliskuuta 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
