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

Ο πρωτότυπος πηγαίος κώδικας του υπολογιστή πλοήγησης (guidance computer) του Apollo 11 (AGC) για το Command Module (Comanche055) και το Lunar Module (Luminary099). Ψηφιοποίηση από ανθρώπους στο [Virtual AGC][3] και το [MIT Museum][4]. Ο στόχος είναι αυτό το αποθετήριο να παρέχει όλο τον πηγαίο κώδικα του Apollo 11. Ως εκ τούτου, τα PRs είναι ευπρόσδεκτα για τυχόν ζητήματα που εντοπίζονται μεταξύ των μεταγραφών σε αυτό το αποθετήριο και των αρχικών σαρώσεων προέλευσης για το [Luminary 099][5] και το [Comanche 055][6], καθώς επίσης και για αρχεία που λείπουν.

## Συνεισφορά

Παρακαλώ διαβάστε το [CONTRIBUTING.gr.md][7] πριν ανοίξετε ένα pull request.

## Σύνταξη πηγαίου κώδικα (Compiling)

Αν ενδιαφέρεστε να κάνετε compile τον πρωτότυπο πηγαίο κώδικα, δείτε το [Virtual AGC][8].

## Απόδοση

&nbsp;                | &nbsp;
:-------------        | :-----
Πνευματική ιδιοκτησία | Δημόσιος τομέας (Public domain)
Comanche055           | Μέρος του πηγαίου κώδικα για το Colossus 2A, το Command Module's (CM) Apollo Guidance Computer (AGC) για το Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099           | Μέρος του πηγαίου κώδικα για το Luminary 1A, το Lollar Module's (LM) Apollo Guidance Computer (AGC) για το Apollo 11<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Συμβολομεταφράστης    | yaYUL
Επικοινωνία           | Ron Burkey <info@sandroid.org>
Ιστοσελίδα            | www.ibiblio.org/apollo
Ψηφιοποίηση           | Αυτός ο πηγαίος κώδικας έχει μεταγραφεί ή αλλιώς προσαρμοστεί από ψηφιοποιημένες εικόνες έντυπου αντιγράφου από το Μουσείο MIT. Η ψηφιοποίηση πραγματοποιήθηκε από τον Paul Fjeld και διευθετήθηκε από την Deborah Douglas του Μουσείου. Ευχαριστώ πολύ και τους δύο.

### Σύμβαση και εγκρίσεις

Προέρχεται από το [CONTRACT_AND_APPROVALS.agc]*

Το πρόγραμμα του AGC αναφέρεται επίσης και ως Colossus 2A.

Αυτό το πρόγραμμα προορίζεται για χρήση στο CM, όπως ορίζεται στην αναφορά R-577. Αυτό το πρόγραμμα προετοιμάστηκε στο πλαίσιο του έργου DSR 55-23870, που χρηματοδοτήθηκε από το Manned Spacecraft Center of the National Aeronautics and Space Administration μέσω της σύμβασης NAS 9-4065 με το Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass.

Υποβλήθηκε από       | Ρόλος | Ημερομηνία
:------------------- | :--- | :---
Margaret H. Hamilton | Colossus Programming Leader<br>Apollo Guidance and Navigation | 28 Mar 69

Εγκρίθηκε από     | Ρόλος | Ημερομηνία
:---------------- | :--- | :---
Daniel J. Lickly  | Διευθυντής, Mission Program Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Fred H. Martin    | Colossus Project Manager<br>Apollo Guidance and Navigation Program | 28 Mar 69
Norman E. Sears   | Διευθυντής, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Richard H. Battin | Διευθυντής, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
David G. Hoag     | Διευθυντής<br>Apollo Guidance and Navigation Program | 28 Mar 69
Ralph R. Ragan    | Αναπληρωτής Διευθυντής<br>Instrumentation Laboratory | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.gr.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
