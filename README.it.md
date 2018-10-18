# Apollo-11
[![NASA][1]][2]

:crossed_flags:
[Bahasa Indonesia][ID],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
**Italiano**,
[Português][PT_BR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR],
[हिंदी][HI_IN]


[ID]:README.id.md
[DE]:README.de.md
[EN]:README.md
[ES]:README.es.md
[IT]:README.it.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md
[HI_IN]:README.hi_in.md

Codice sorgente dell'Apollo 11 Guidance Computer (AGC) utilizzato
dal Command Module (Comanche055) e dal Lunar Module (Luminary099).
Digitalizzato dagli autori del [Virtual AGC][3] e dal [MIT Museum][4].
L'obbiettivo è quello di avere una repository contente il codice
usato dalla missione Apollo 11. Di conseguenza, le PRs volte a
corregere qualsiasi problema, identificato confrontando le
trascrizioni in questa repository e le scansioni originali del
codice del [Luminary 099][5] e del [Comanche 055][6], sono benvenute.

## Contribuire
Leggere [CONTRIBUTING.md][7] prima di aprire una pull request (PR).

## Compilare
Se sei interessato a compilare il codice sorgente originale
dai un'occhiata a [Virtual AGC][8].

## Crediti

&nbsp;           | &nbsp;
:-------------   | :-----
Copyright        | Dominio Pubblico
Comanche055      | Parte del codice sorgente del Colossus 2A, il modulo di comando (CM) dell'Apollo Guidance Computer (AGC) dell'Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099      | Parte del codice sorgente del Luminary 1A, il modulo lunare (LM) dell'Apollo Guidance Computer (AGC) dell'Apollo 11<br>`Assemble revision 001 of AGC program LYM99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembler        | yaYUL
Contatti         | Ron Burkey <info@sandroid.org>
Website          | www.ibiblio.org/apollo
Digitalizzazione | Questo codice sorgente è stato trascritto utilizzando le copie cartace del codice sorgente presenti al MIT Museum. La digitalizzazione è stata svolta da Paul Fjeld ed organizzata da Deborah Douglas del MIT Museum. Grazie mille ad entrambi.

### Contratto e Approvazioni
*Derivato da [CONTRACT_AND_APPROVALS.agc]*

Questo AGC deve fare riferimento al Colossus 2A.

Questo programma è destinato all'uso nel CM (Command Module) come specificato nel report `R-577`. Questo programma è stato sviluppato nell'ambito del progetto DSR `55-23870`, promosso dal Manned Spacecraft Center of The National Aeronautics and Space Administration attraverso il contratto `NAS 9-4065` siglato con l'Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass.

Presentato da         | Ruolo | Data
:-------------------- | :---- | :---
Margaret H. Hamilton  | Colossus Programming Leader<br>Apollo Guidance and Navigation | 28 Mar 69

Approvato da       | Ruolo | Data
:----------------- | :---- | :---
Daniel J. Lickly   | Director, Mission Program Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Fred H. Martin     | Colossus Project Manager<br>Apollo Guidance and Navigation Program | 28 Mar 69
Norman E. Sears    | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Richard H. Battin  | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
David G. Hoag      | Director<br>Apollo Guidance and Navigation Program | 28 Mar 69
Ralph R. Ragan     | Deputy Director<br>Instrumentation Laboratory | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
