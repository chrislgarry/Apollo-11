# Menyumbang

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
[tiếng Việt][VI],
[Türkçe][TR],
[Ελληνικά][GR],
[Українська][UK]،
[العربية][AR],
[हिन्दी][HI_IN],
[한국어][KO_KR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN]

[AR]:CONTRIBUTING.ar.md
[AZ]:CONTRIBUTING.az.md
[CA]:CONTRIBUTING.ca.md
[CZ]:CONTRIBUTING.cz.md
[DA]:CONTRIBUTING.da.md
[DE]:CONTRIBUTING.de.md
[EN]:../CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[GL]:CONTRIBUTING.gl.md
[GR]:CONTRIBUTING.gr.md
[HI_IN]:CONTRIBUTING.hi_in.md
[ID]:CONTRIBUTING.id.md
[IT]:CONTRIBUTING.it.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[LT]:CONTRIBUTING.lt.md
[MN]:CONTRIBUTING.mn.md
[NL]:CONTRIBUTING.nl.md
[NO]:CONTRIBUTING.no.md
[PL]:CONTRIBUTING.pl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[UK]:CONTRIBUTING.uk.md
[VI]:CONTRIBUTING.vi.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Kode asli dalam repositori ini didigitalkan secara manual dari kertas cetak, sehingga inkonsistensi dan ketidaksesuaian lainnya mungkin terjadi karena kesalahan. Kode perlu diubah agar sesuai dengan edisi yang diterbitkan:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

Situs web berikut dapat digunakan untuk menavigasi dengan mudah hasil cetakan pindaian untuk Comanche dan Luminary: https://28gpc.csb.app/

## Ekstensi yang berguna

GitHub memiliki dukungan sintaks untuk pengaturan bahasa AGC. Sayangnya kode editor anda tidak dapat melakukannya, namun ada ekstensi bahasa AGC yang menyediakan penyorotan sintaks untuk kode editor berikut:

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

† Mendukung pemformatan otomatis

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

## Pemformatan

**Note:** GitHub dan plugin yang tercantum di atas akan secara otomatis mengonfirmasi bahwa Anda menggunakan plugin yang benar.

- Gunakan panjang tab
- Gunakan panjang tab 8
- Hapus karakter spasi ataupun tab di awal atau di akhir string

## Apa yang saya periksa?

Setiap perbedaan antara pemindaian dan kode sumber dalam repositori ini.

### Komentar

Komentar dalam kode yang ditranskripsikan **HARUS** cocok dengan pindaian **persis**.

Masalah umum yang harus Anda perhatikan saat pemeriksaan termasuk, tetapi tidak terbatas pada:

#### Kesalahan Tipografi

Di beberapa tempat, pengembang asli membuat kesalahan ketik saat menulis komentar. Beberapa di antaranya salah dikoreksi selama digitalisasi awal, namun digitalisasi juga menimbulkan kesalahan tipografi yang tidak ada dalam pemindaian.

Misalnya, jika komentar digital berisi `SPACECRAFT`, tetapi `spaecraft` dicetak dalam pemindaian, maka digitalisasi **HARUS** dikoreksi ke `spaecraft` (hilang `C`).

Demikian juga, jika sebuah kata memiliki kesalahan ketik dalam digitalisasi tetapi dieja dengan benar dalam pemindaian, maka kesalahan ketik **HARUS** diperbaiki.

#### Spasi

Spasi antara dua karakter dalam komentar **HARUS** cocok dengan pindaian. Dalam kebanyakan kasus (lihat diskusi di [#316][10]), ini adalah:

- Satu spasi untuk kata-kata baru.
- Spasi ganda untuk kalimat baru.
- Tiga ruang untuk lekukan.

Tidak semua halaman dalam pemindaian mengikuti generalisasi ini, jika pemindaian hanya memiliki satu spasi, bukan spasi ganda, gunakan satu spasi.

### Jeda baris

- Pemisahan baris *dengan* `R0000` di kolom 1 harus sama persis dengan pemindaian.
- Pemisahan baris *dengan**__out__* `R0000` di kolom 1 hanya boleh berisi 1 atau 2 baris kosong dalam satu baris.
  - Jika ada lebih dari 2 jeda baris kosong, hapus jeda baris tambahan.
    - Baris dengan `R0000` di kolom 1 tidak diperhitungkan dalam hal ini.
  - Dalam gambar sumber, ini dibuat oleh digit yang belum dicetak di kolom 8. A 2 di sana memaksa spasi ganda (satu baris kosong dan 3 memaksa spasi tiga (garis kosong ganda). Nilai 4-8 didefinisikan tetapi tidak pernah digunakan. Baca selengkapnya di [#159][7]

Misalnya berikut ini:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Harus menjadi:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Catatan

Sebelum Anda membuat PR, pastikan perubahan Anda konsisten dengan pemindaian!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
