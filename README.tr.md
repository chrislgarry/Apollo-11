# Apollo-11
[![NASA][1]][2]

:crossed_flags:
[Bahasa Indonesia][ID],
[Català][CA],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Italiano][IT],
[Português][PT_BR],
[Русский][RU],
**Türkçe**,
[العربية][AR],
[हिंदी][HI_IN],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR],
[日本][JA]

[AR]:README.ar.md
[ID]:README.id.md
[CA]:README.ca.md
[DE]:README.de.md
[EN]:README.md
[ES]:README.es.md
[IT]:README.it.md
[FR]:README.fr.md
[JA]:README.ja.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md
[HI_IN]:README.hi_in.md
[RU]:README.ru.md

Komuta Modülü (Comanche055) ve Ay Modülü (Luminary099) için Orijinal Apollo 11 kontrol bilgisayarı 
(Apollo 11 guidance computer -AGC-) kaynak kodu.  [Virtual AGC][3] ve [MIT Müzesi][4]'ndeki insanlar tarafından 
dijital ortama aktarıldı. Amaç, orijinal Apollo 11 kaynak kodu için bir repo olmasıdır. Bu nedenle bu repodaki 
transkripsiyonlar ve [Luminary099][5]  [Comanche 055][6] için orijinal kaynak taramaları arasında tespit edilen 
tüm sorunlar - ve bunların yanı sıra burada saymadığım gözden kaçırmış olabileceğim herhangi bir dosya - için 
Pull Request atabilirsiniz.  

## Katkıda Bulunma

Pull request atmadan önce lütfen [CONTRIBUTING.md][7] yazısını okuyunuz. 

## Derleme

Orijinal kaynak kodu derleme ile ilgileniyorsanız [Virtual AGC][8] ‘ye göz atın.

## Atıflar

&nbsp;         | &nbsp;
:------------- | :-----
Telif Hakkı     | Kamu Malı
Comanche055    | Apollo 11 için Komuta Modülünün (CM), Apollo Kontrol Bilgisayarı (AGC),Colossus 2A kaynak kodunun bir bölümü <br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099    | Ay Modülünün (LM), Apollo 11 için Apollo Kontrol Bilgisayarı (AGC) ve Luminary1A kaynak kodunun bir bölümü<br>`Assemble revision 001 of AGC program LYM99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembler      | yaYUL
İletişim        | Ron Burkey <info@sandroid.org>
Websitesi        | www.ibiblio.org/apollo
Dijital Ortama Aktarma | Bu kaynak kod MIT Müzesi'nden bir basılı kopyanın dijital ortama aktarılmış görüntülerinden kopyalanmış veya başka şekilde uyarlanmıştır. Dijital ortama aktarma Paul Fjeld tarafından yapıldı ve müze çalışanı Deborah Douglas tarafından düzenlendi. Her ikisine de çok teşekkürler.

## Sözleşme ve Onaylar

*[CONTRACT_AND_APPROVALS.agc] 'den sağlandı*

Bu program CM'de, `R-577` raporunda belirtilen şekilde kullanılmak üzere tasarlanmıştır. Bu program DSR projesi `55-23870` kapsamında `NAS 9-4065` sözleşmesiyle Instrumentation Laboratuvarı, Massachusetts Teknoloji Enstitüsü, Cambridge, Mass ve Ulusal Havacılık ve Uzay İdaresi İnsanlı Uzay Aracı Merkezi sponsorluğu ile hazırlanmıştır.

Gönderen          | Rolü | Tarih
:-------------------- | :--- | :---
Margaret H. Hamilton  | Colossus Programlama Lideri<br>Apollo Kontrol ve Navigasyon| 28 Mar 69

Onaylayan        | Rolü | Tarih
:----------------- | :--- | :---
Daniel J. Lickly   | Yönetici, Görev Programı Geliştirme<br>Apollo Kontrol ve Navigasyon Programı | 28 Mar 69
Fred H. Martin     | Colossus Proje Yöneticisi<br>Apollo Kontrol ve Navigasyon Programı | 28 Mar 69
Norman E. Sears    | Yönetici, Görev Geliştirme<br>Apollo Kontrol ve Navigasyon Programı | 28 Mar 69
Richard H. Battin  | Yönetici, Görev Geliştirme<br>Apollo Kontrol ve Navigasyon Programı | 28 Mar 69
David G. Hoag      | Yönetici<br>Apollo Kontrol ve Navigasyon Programı | 28 Mar 69
Ralph R. Ragan     | Yönetici Yardımcısı<br>Instrumentation Laboratuvarı | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://rawcdn.githack.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc

