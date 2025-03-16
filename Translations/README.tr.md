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

Orijinal Apollo 11 Yönlendirme Bilgisayarı'nın (AGC) Komuta Modülü (Comanche055) ve Ay Modülü (Luminary099)'nün kaynak kodu. [Virtual AGC][3] ve [MIT Museum][4] çalışanları tarafından sayısallaştırılmıştır. Amaç orijinal Apollo 11 kaynak kodunu içeren bir repo oluşturmak. Bu yüzden, bu repodaki transkriptler ile orijinal [Luminary 099][5] ve [Comanche 055][6] kaynak taramaları arasında tespit edilen hatalarla ilgili veya benim kaçırdığım herhangi bir dosya hakkında yollanacak pull request'lere açığız.

## Katkıda Bulunma

Lütfen pull request açmadan [CONTRIBUTING.tr.md][7] dosyasını okuyun.

## Derleme

Eğer orijinal kaynak kodu derlemek isterseniz, [Virtual AGC][8] projesine bakabilirsiniz.

## Nitelikler

&nbsp;          | &nbsp;
:-------------- | :-----
Lisans          | Kamu malı - ABD hükümeti çalışması
Comanche055     | Apollo Yönlendirme Bilgisayarı (AGC)'nin Komuta Modülü (CM) olan Colossus 2A'nın kaynak kodunun bir parçası.<br>`AGC programı Comanche'nin NASA tarafından yapılan 055 sayılı birleştirme revizyonu`<br>`2021113-051. 10:28 NİS. 1, 1969`
Luminary099     | Apollo Yönlendirme Bilgisayarı (AGC)'nin Ay Modülü (LM) olan Luminary 1A'in kaynak kodunun bir parçası.<br>`AGC Programı LMY99'un NASA tarafından yapılan 001 sayılı birleştirme revizyonu`<br>`2021112-061. 16:27 TEM. 14, 1969`
Derleyici       | yaYUL
İletişim        | Ron Burkey <info@sandroid.org>
Internet Sitesi | www.ibiblio.org/apollo
Sayısallaştırma | Bu kaynak kodu MIT Müzesi'ndeki basılı kopyaların fotoğraflarından uyarlanmış veya aktarılmıştır. Sayısallaştırma Müze çalışanı Deborah Douglas tarafından ayarlanmış, Paul Fjeld tarafından yapılmıştır. İkisine de çok teşekkürler.

### İletişim ve Onaylar

*[CONTRACT_AND_APPROVALS.agc]'den alınmıştır.*

Bu AGC programı Colossus 2A olarak da bilinir.

Rapor `R-577`'de belirtildiği gibi, bu program CM için kullanım amacıyla hazırlanmıştır. Bu program DSR `55-23870` projesi altında, Ulusal Havacılık ve Uzay İdaresi Uzay Aracı Merkezi'nin sponsorluğunda, Cambridge Mass Massachusetts Teknoloji Enstitüsü Enstrümantasyon Laboratuvarı ile yapılan `NAS 9-4065` anlaşması ile hazırlanmıştır.

Gönderen             | Mevkisi | Tarih
:------------------- | :------ | :----
Margaret H. Hamilton | Colossus Programlama Lideri<br>Apollo Yönlendirme ve Navigasyon | 28 Mar 69

Onaylayan         | Mevkisi | Tarih
:---------------- | :------ | :----
Daniel J. Lickly  | Direktör, Görev Programı Geliştirme<br>Apollo Yönlendirme ve Navigasyon Programı | 28 Mart 1969
Fred H. Martin    | Colossus Proje Yöneticisi<br>Apollo Yönlendirme ve Navigasyon Programı | 28 Mart 1969
Norman E. Sears   | Direktör, Görev Geliştirme<br>Apollo Yönlendirme ve Navigasyon Programı | 28 Mart 1969
Richard H. Battin | Direktör, Görev Geliştirme<br>Apollo Yönlendirme ve Navigasyon Programı | 28 Mart 1969
David G. Hoag     | Direktör<br>Apollo Yönlendirme ve Navigasyon Programı | 28 Mart 1969
Ralph R. Ragan    | Direktör Yardımcısı<br>Enstrümantasyon Laboratuvarı | 28 Mart 1969

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://flat.badgen.net/badge/NASA/Mission%20Overview/0B3D91
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/Translations/CONTRIBUTING.tr.md
[8]:https://github.com/rburkey2005/virtualagc
[SWH]:https://flat.badgen.net/badge/Software%20Heritage/Archive/0B3D91
[SWH_URL]:https://archive.softwareheritage.org/browse/origin/https://github.com/chrislgarry/Apollo-11/
[Comanche]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/1
[ComancheMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/1
[Luminary]:https://flat.badgen.net/github/milestones/chrislgarry/Apollo-11/2
[LuminaryMilestone]:https://github.com/chrislgarry/Apollo-11/milestone/2
