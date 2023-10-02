🎌
[Bahasa Indonesia][ID],
[Català][CA],
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
**English**,
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Română][RO],
[Tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Русский][RU],
[العربية][AR],
[فارسی][FA],
[हिंदी][HI_IN],
[অসমীয়া][AS_IN],
[বাংলা][BD_BN],
[မြန်မာ][MM],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:README.ar.md
[AS_IN]:README.as_in.md
[BD_BN]:README.bd_bn.md
[CA]:README.ca.md
[CZ]:README.cz.md
[DA]:README.da.md
[DE]:README.de.md
[EN]:README.md
[ES]:README.es.md
[FA]:README.fa.md
[FR]:README.fr.md
[GR]:README.gr.md
[HI_IN]:README.hi_in.md
[ID]:README.id.md
[IT]:README.it.md
[JA]:README.ja.md
[KO_KR]:README.ko_kr.md
[KU]:README.ku.md
[LT]:README.lt.md
[MM]:README.mm.md
[NL]:README.nl.md
[NO]:README.no.md
[PL]:README.pl.md
[PT_BR]:README.pt_br.md
[RO]:README.ro.md
[RU]:README.ru.md
[TR]:README.tr.md
[VI]:README.vi.md
[ZH_CN]:README.zh_cn.md
[ZH_TW]:README.zh_tw.md

---

# Apollo 11 Guidance Computer Source Code

![NASA Mission Overview][badge_nasa]
[![Software Heritage][badge_swh]][swh_url]
[![Comanche Milestone][badge_comanche]][comanche_milestone]
[![Luminary Milestone][badge_luminary]][luminary_milestone]

Welcome to the repository containing the original source code for the guidance computers used in the historic Apollo 11 mission—the first manned moon landing in 1969. The source code for the Command Module (Comanche055) and Lunar Module (Luminary099) is meticulously digitized and made available here, aiming to preserve a crucial part of technological history.

## Table of Contents

- [Contributing](#contributing)
- [Compiling Instructions](#compiling-instructions)
- [Attribution](#attribution)
- [Contact](#contact)
- [Additional Resources](#additional-resources)

## Contributing

We welcome contributions from the community to enhance the accuracy and completeness of the source code transcriptions. If you identify any discrepancies between the transcriptions here and the original source scans, or if you find any missing files, please review our [Contribution Guidelines](CONTRIBUTING.md) to open a pull request.

## Compiling Instructions

If you are interested in compiling the original source code, you can follow the instructions provided in the [Virtual AGC repository][virtual_agc_repo]. The Virtual AGC project offers the tools and environment needed to compile and run this historic code.

## Attribution

- **Copyright**: Public domain
- **Comanche055**: Part of the source code for Colossus 2A, the Command Module's (CM) Apollo Guidance Computer (AGC) for Apollo 11. Assemble revision 055 of AGC program Comanche by NASA. Dated April 1, 1969.
- **Luminary099**: Part of the source code for Luminary 1A, the Lunar Module's (LM) Apollo Guidance Computer (AGC) for Apollo 11. Assemble revision 001 of AGC program LMY99 by NASA. Dated July 14, 1969.
- **Assembler**: yaYUL
- **Contact**: Ron Burkey (info@sandroid.org)
- **Website**: [Apollo Guidance Computer at ibiblio][apollo_guidance_computer]
- **Digitalisation**: This source code has been transcribed or adapted from digitized images of a hardcopy from the MIT Museum. The digitization was performed by Paul Fjeld and arranged for by Deborah Douglas of the Museum.

### Contract and Approvals

Derived from [CONTRACT_AND_APPROVALS.agc](CONTRACT_AND_APPROVALS.agc).

This AGC program is referred to as Colossus 2A and was intended for use in the CM as specified in report `R-577`. The program was prepared under DSR project `55-23870`, sponsored by the Manned Spacecraft Center of The National Aeronautics and Space Administration through contract `NAS 9-4065` with the Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass.

## Contact

For inquiries or more information, feel free to reach out:
- Ron Burkey (info@sandroid.org)

## Additional Resources

- [NASA Mission Overview][nasa_mission_overview]
- [Software Heritage Archive][swh_url]
- [Comanche Milestone][comanche_milestone]
- [Luminary Milestone][luminary_milestone]

[badge_nasa]: https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[badge_swh]: https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[badge_comanche]: https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[badge_luminary]: https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[swh_url]: https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[comanche_milestone]: https://github.com/chrislgarry/Apollo-11/milestone/1
[luminary_milestone]: https://github.com/chrislgarry/Apollo-11/milestone/2
[virtual_agc_repo]: https://github.com/rburkey2005/virtualagc
[apollo_guidance_computer]: http://www.ibiblio.org/apollo
[nasa_mission_overview]: https://www.nasa.gov/history/apollo-11-mission-overview/
