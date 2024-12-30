# Аполлон-11

[![NASA][1]][2]
[![SWH]][SWH_URL]
[![Comanche]][ComancheMilestone]
[![Luminary]][LuminaryMilestone]

🎌
[Bahasa Indonesia][ID],
[Català][CA],
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Kurdi][KU],
[Mongolian][MN],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Română][RO],
[Tiếng Việt][VI],
[Türkçe][TR],
[Ukrainian][UA],
[Ελληνικά][GR],
**Русский**,
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
[MN]:README.mn.md
[NL]:README.nl.md
[NO]:README.no.md
[PL]:README.pl.md
[PT_BR]:README.pt_br.md
[RO]:README.ro.md
[RU]:README.ru.md
[TR]:README.tr.md
[UA]:README.ua.md
[VI]:README.vi.md
[ZH_CN]:README.zh_cn.md
[ZH_TW]:README.zh_tw.md

Оригинальный исходный код бортового управляющего компьютера Аполлон 11 (AGC) для командного модуля (Comanche055) и лунного модуля (Luminary099). Оцифровано людьми из [Virtual AGC][3] и [MIT Museum][4]. Цель - это создание репозитория с оригинальным исходным кодом миссии Аполлон 11. Таким образом приветствуются pull requests (PRs) с исправлениями для любых найденных ошибок в файлах этого репозитория и оригинальных сканах исходного кода для [Luminary 099][5] и [Comanche 055][6], так же как и для любых файлов, которые я мог пропустить.

## Сотрудничество

Пожалуйста, прочитайте [CONTRIBUTING.md][7] перед тем как открывать pull request.

## Компиляция

Если вы заинтересованы в компилировании данного исходного кода, смотрите инструкции здесь [Virtual AGC][8].

## Атрибуция

&nbsp;          | &nbsp;
:-------------- | :-----
Авторское право | Общественное достояние
Comanche055     | Часть исходного кода для Colossus 2A, командного модуля (CM) бортового управляющего компьютера Аполлон (AGC) для миссии Аполлон 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 Апрель. 1, 1969`
Luminary099     | Часть исходного кода для Luminary 1A, лунного модуля (LM) бортового управляющего компьютера Аполлон (AGC) для миссии Аполлон 11<br>`Assemble revision 001 of AGC program LMY99 by NASA`<br>`2021112-061. 16:27 Июль. 14, 1969`
Ассемблер       | yaYUL
Контакты        | Ron Burkey <info@sandroid.org>
Вебсайт         | www.ibiblio.org/apollo
Оцифровка       | Исходный код был расшифрован или иным способом адаптирован из оцифрованных изображений печатных копий из музея университета MIT. Оцифровку выполнил Paul Fjeld, упорядочила Deborah Douglas из музея. Огромная благодарность обоим.

### Договор и разрешения

*Получено из [CONTRACT_AND_APPROVALS.agc]*

Эта AGC программа также должна быть упомянута как Colossus 2A.

Эта программа предназначена для использования в командном модуле (CM) как указано в отчете `R-577`. Программа была подготовлена в рамках проекта DSR `55-23870`, при поддержке центра пилотируемых космических кораблей NASA по договору `NAS 9-4065` с участием лаборатории приборостроения, Массачусетского технологического университета (MIT), Кембридж, Массачусетс.

Представлено         | Роль | Дата
:------------------- | :--- | :---
Margaret H. Hamilton | Ведущий программист Colossus<br>Управление и навигация Аполлон | 28 Март 69

Подтверждено      | Роль | Дата
:---------------- | :--- | :---
Daniel J. Lickly  | Директор, Разработка программы миссии<br>Программа управления и навигации Аполлона | 28 Март 69
Fred H. Martin    | Проектный менеджер Colossus<br>Программа управления и навигации Аполлона | 28 Март 69
Norman E. Sears   | Директор, Разработка миссии<br>Программа управления и навигации Аполлона | 28 Март 69
Richard H. Battin | Директор, Разработка миссии<br>Программа управления и навигации Аполлона | 28 Март 69
David G. Hoag     | Директор<br>Программа управления и навигации Аполлона | 28 Март 69
Ralph R. Ragan    | Заместитель директора<br>Лаборатория приборостроения| 28 Март 69

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
