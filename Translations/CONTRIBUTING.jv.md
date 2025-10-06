# Nyumbang

🌐
[Azerbaijani][AZ],
[bahasa Indonesia][ID],
[Català][CA]،
[Čeština][CZ],
[Dansk][DA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Galego][GL],
[Italiano][IT],
[Kurdi][KU],
[Kurdî][KU],
[Lietuvių][LT],
[Mongolia][MN],
[Nederlands][NL],
[Norsk][NO],
[Polski][PL],
[Português][PT_BR],
[Svenska][SV],
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Українська][UK]،
[العربية][AR],
[हिन्दी][HI_IN],
[한국어][KO_KR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[Basa Jawa][JV]

[AR]:Translations/CONTRIBUTING.ar.md
[AZ]:Translations/CONTRIBUTING.az.md
[CA]:Translations/CONTRIBUTING.ca.md
[CZ]:Translations/CONTRIBUTING.cz.md
[DA]:Translations/CONTRIBUTING.da.md
[DE]:Translations/CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:Translations/CONTRIBUTING.es.md
[FR]:Translations/CONTRIBUTING.fr.md
[GL]:Translations/CONTRIBUTING.gl.md
[GR]:Translations/CONTRIBUTING.gr.md
[HI_IN]:Translations/CONTRIBUTING.hi_in.md
[ID]:Translations/CONTRIBUTING.id.md
[IT]:Translations/CONTRIBUTING.it.md
[JA]:Translations/CONTRIBUTING.ja.md
[JV]:Translations/CONTRIBUTING.jv.md
[KO_KR]:Translations/CONTRIBUTING.ko_kr.md
[KU]:Translations/CONTRIBUTING.ku.md
[LT]:Translations/CONTRIBUTING.lt.md
[MN]:Translations/CONTRIBUTING.mn.md
[NL]:Translations/CONTRIBUTING.nl.md
[NO]:Translations/CONTRIBUTING.no.md
[PL]:Translations/CONTRIBUTING.pl.md
[PT_BR]:Translations/CONTRIBUTING.pt_br.md
[SV]:Translations/CONTRIBUTING.sv.md
[TR]:Translations/CONTRIBUTING.tr.md
[UK]:Translations/CONTRIBUTING.uk.md
[VI]:Translations/CONTRIBUTING.vi.md
[ZH_CN]:Translations/CONTRIBUTING.zh_cn.md
[ZH_TW]:Translations/CONTRIBUTING.zh_tw.md

Kode sumber ing repositori iki wis didigitalisasi kanthi manual saka printout kertas, mula kadhang kala ana salah tulis utawa beda cilik sing ora disengaja. Kode kudu didandani supaya konsisten karo printout scan asli iki:

- [AGC printouts kanggo Comanche][8]
- [AGC printouts kanggo Luminary][9]

Situs iki bisa digunakake kanggo nggampangaké navigasi ing antarane printout sing wis discan kanggo Comanche lan Luminary: https://28gpc.csb.app/

## Ekstensi sing Migunani

GitHub wis nduwé dhukungan sintaks kanggo basa assembly AGC bawaan. Nanging umume editor kode ora nduwé. Untungé, ana ekstensi AGC sing nyedhiyakake **syntax highlighting** kanggo sawetara editor, kayata:

- [Atom][Atom]†
- [CodeBlocks][CodeBlocks]
- [Eclipse][Eclipse]
- [Kate][Kate]
- [ProgrammersNotepad][ProgrammersNotepad]
- [Sublime Text 3][Sublime Text]†
- [TextPad][TextPad]
- [Vim][Vim]
- [Visual Studio Code][VisualStudioCode]†
- [jEdit][jEdit]

† Dhukungan format otomatis

[Atom]:https://github.com/Alhadis/language-agc
[CodeBlocks]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/CodeBlocks
[Eclipse]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Eclipse
[Kate]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/Kate
[ProgrammersNotepad]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/ProgrammersNotepad
[Sublime Text]:https://github.com/jimlawton/AGC-Assembly
[TextPad]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/TextPad
[Vim]:https://github.com/wsdjeg/vim-assembly
[VisualStudioCode]:https://github.com/wopian/agc-assembly
[jEdit]:https://github.com/virtualagc/virtualagc/tree/master/Contributed/SyntaxHighlight/jEdit

## Format

**Cathetan:** GitHub lan ekstensi sing wis ditandai † ing dhuwur bakal mesthekake formatmu wis bener.

- Gunakake tab kanggo indentasi
- Jembar tab = 8
- Mbuwang spasi kosong ing mburi baris

## Opo sing kudu dicek?

Kabeh bedane antarane scan lan kode sumber ing repositori iki.

### Komentar

Komentar ing kode sing wis didigitalisasi **KUDU** padha persis kaya ing scan.

Masalah umum sing kudu diwaspadai nalika mriksa, kalebu nanging ora winates ing:

#### Salah Tulis (Typo)

Kadhang kala para pangembang asli nggawe salah tulis ing komentar. Sawetara salah tulis iki salahé wis “dibeneraké” nalika digitalisasi awal, nanging proses digitalisasi uga ngenalaké salah tulis anyar sing ora ana ing scan.

Conto: yen komentar digitalisasi ana `SPACECRAFT`, nanging ing scan ditulis `SPAECRAFT`, mula kudu dibalekaké dadi `SPAECRAFT`.

#### Spasi

Spasi ing antarane karakter utawa tembung ing komentar **KUDU** padha karo scan. Ing umume kasus (ndeleng diskusi ing [#316][10]) aturané yaiku:

- Siji spasi kanggo tembung anyar.
- Loro spasi kanggo kalimat anyar.
- Telu spasi kanggo indentasi.

Nanging ora kabeh kaca ing scan konsisten, yen mung ana siji spasi ing scan, ya kudu nganggo siji spasi.

### Pamisah Baris

- Pamisah baris **sing nganggo** `R0000` ing kolom 1 kudu padha karo scan.  
- Pamisah baris **tanpa** `R0000` ing kolom 1 mung kena 1 utawa 2 baris kosong berturut-turut.  
  - Yen luwih saka 2, kudu dibusak sing luwih.  
  - Ing gambar sumber, iki asale saka digit sing ora dicithak ing kolom 8. Angka 2 nyebabake spasi ganda (1 baris kosong), angka 3 spasi telu (2 baris kosong). Angka 4-8 ana definisi nanging ora tau digunakake. Waca luwih lengkap ing [#159][7].

Conto:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Kudu dadi:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Cathetan

Sadurunge nggawe PR, priksa maneh supaya owahanmu konsisten karo hasil scan!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741