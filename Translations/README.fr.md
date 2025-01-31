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

Code source original de l'ordinateur de guidage Apollo 11 (AGC) pour le module de commande (Comanche055) et le module lunaire (Luminary099). Numérisé par les gens du [Virtual AGC][3] et du [MIT Museum][4]. Le but est d'être un repo pour le code source original d'Apollo 11. En tant que tel, les PR sont les bienvenues pour tous les problèmes identifiés entre les transcriptions dans ce référentiel et les scans source originaux pour [Luminary 099][5] et [Comanche 055][6], ainsi que pour tous les fichiers que j'ai pu manquer.

## Contribuer

Merci de lire [CONTRIBUTING.fr.md][7] avant d'ouvrir une pull request.

## Compilation

Si vous êtes intéressé par la compilation du code source original, visitez [Virtual AGC][8].

## Attribution

&nbsp;        | &nbsp;
:------------ | :-----
Copyright     | Domaine public
Comanche055   | Partie du code source de Colossus 2A, le module de commande (CM) Ordinateur embarqué de navigation (AGC), pour Apollo 11.<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099   | Partie du code source de Luminary 1A, le module de lunaire (LM) Ordinateur embarqué de navigation (AGC), pour Apollo 11.<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembleur    | yaYUL
Contact       | Ron Burkey <info@sandroid.org>
Site internet | www.ibiblio.org/apollo
Numérisation  | Ce code source a été transcrit ou autrement adapté à partir de données numérisées des images d'une copie papier du Musée du MIT. La numérisation a été effectuée par Paul Fjeld, et organisé par Deborah Douglas du Musée. Un grand merci à eux deux.

### Contrat et Approbations

*Dérivé de [CONTRACT_AND_APPROVALS.agc]*

```plain
CE PROGRAMME DE L'AGC SERA ÉGALEMENT DÉSIGNÉ SOUS LE NOM DE :
    COLOSSUS 2A

CE PROGRAMME EST DESTINÉ À ÊTRE UTILISÉ EN CM COMME INDIQUÉ
DANS LE RAPPORT R-577.  CE PROGRAMME A ÉTÉ PRÉPARÉ DANS LE CADRE
DU DSR PROJET 55-23870, PARRAINÉ PAR L'ENGIN SPATIAL HABITÉ
CENTRE NATIONAL DE L'AÉRONAUTIQUE ET DE L'ADMINISTRATION SPATIALE
PAR LE BIAIS DU CONTRAT NAS 9-4065 AVEC LE LABORATOIRE
D'INSTRUMENTATION, INSTITUT DE TECHNOLOGIE DU MASSACHUSETTS,
CAMBRIDGE, MASS.
```

Soumis par           | Rôle | Date
:------------------- | :--- | :---
Margaret H. Hamilton | Chef de programmation Colossus<br>Programme de guidage et de navigation Apollo | 28 Mar 69

Approuvé par      | Rôle | Date
:---------------- | :--- | :---
Daniel J. Lickly  | Directeur, Développement des programmes de mission<br>Programme de guidage et de navigation Apollo | 28 Mar 69
Fred H. Martin    | Chef de projet Colossus<br>Programme de guidage et de navigation Apollo | 28 Mar 69
Norman E. Sears   | Directeur, Développement de la mission<br>Programme de guidage et de navigation Apollo | 28 Mar 69
Richard H. Battin | Directeur, Développement de la mission<br>Programme de guidage et de navigation Apollo | 28 Mar 69
David G. Hoag     | Directeur<br>Programme de guidage et de navigation Apollo | 28 Mar 69
Ralph R. Ragan    | Directeur adjoint<br>Laboratoire d'instrumentation | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.fr.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
