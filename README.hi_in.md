# अपोलो -11
[![NASA][1]][2]

:crossed_flags:
[English][EN],
[Español][ES],
[Français][FR],
[Português][PT_BR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR],
**हिंदी**

[EN]:README.md
[ES]:README.es.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md
[HI_IN]:README.hi_in.md


मूल अपोलो 11 मार्गदर्शन कंप्यूटर (Apollo 11 Guidance computer, AGC) कमांड मॉड्यूल के लिए स्रोत कोड (Comanche055) और चंद्र मॉड्यूल (Luminary099). डिजिटाइज किया गया [Virtual AGC][3] और [MIT Museum][4] के लोगों द्वारा. लक्ष्य मूल अपोलो 11 स्रोत कोड के लिए एक रेपो होना है. इस प्रकार, ट्रांसक्रिप्शन के बीच पहचाने गए किसी भी मुद्दे के लिए पीआर का स्वागत है इस भंडार में और मूल स्रोत स्कैन के लिए [Luminary 099][5] और[Comanche 055][6], साथ ही साथ कोई भी फाइल जो मैंने छोड़ी हो।

## योगदान
कृपया पढ़ें [CONTRIBUTING.md][7] पुल अनुरोध खोलने से पहले।

## संकलन
यदि आप मूल स्रोत कोड संकलित करने में रुचि रखते हैं, चेक आउट [Virtual AGC][8].

## आरोपण

&nbsp;         | &nbsp;
:------------- | :-----
कॉपीराइट      | पब्लिक डोमेन
Comanche055    | कोलोसस 2(Colossus 2) ए के लिए स्रोत कोड का हिस्सा, अपोलो 11(Apollo 11) के लिए कमांड मॉड्यूल (सीएम) अपोलो गाइडेंस कंप्यूटर (एजीसी)<br>`नासा द्वारा एजीसी कार्यक्रम कॉमचेस के समेकित संशोधन 055`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099    |  ल्यूमिनरी 1 ए के स्रोत कोड का हिस्सा , चंद्र मॉड्यूल (एलएम) अपोलो गाइडेंस कंप्यूटर (एजीसी) अपोलो 11 के लिये<br>`नासा द्वारा एजीसी(AGC) कार्यक्रम LYM99 के समेकित संशोधन 001`<br>`2021112-061. 16:27 JUL. 14, 1969`
कोडांतरक     | yaYUL
संपर्क करें        | Ron Burkey <info@sandroid.org>
वेबसाइट        | www.ibiblio.org/apollo
डिजिटलीकरण | यह स्रोत कोड एमआईटी संग्रहालय से हार्डकॉपी की डिजिटली छवियों से प्रतिलिपि या अन्यथा अनुकूलित किया गया है। डिजिटलीकरण Paul Fjeld द्वारा किया गया था, और संग्रहालय के Deborah Douglas द्वारा इसकी व्यवस्था की गई थी। दोनों के लिए बहुत धन्यवाद।

### अनुबंध और स्वीकृतियां
*से व्युत्पन्न [CONTRACT_AND_APPROVALS.agc]*

इस एजीसी कार्यक्रम को कोलोसस 2ए(Colossus 2A.) के रूप में भी जाना जाएगा।

इस कार्यक्रम का उद्देश्य CM में उपयोग के लिए किया गया है जैसा रिपोर्ट `R-577` में निर्दिष्ट है यह कार्यक्रम डीएसआर परियोजना `55-23870` के तहत तैयार किया गया था , इंस्ट्रुमेंटेशन प्रयोगशाला, मैसाचुसेट्स इंस्टीट्यूट ऑफ टेक्नोलॉजी, कैम्ब्रिज, मास के साथ अनुबंध `NAS 9-4065` के माध्यम से राष्ट्रीय एयरोनॉटिक्स और अंतरिक्ष प्रशासन के मानव अंतरिक्षयान केंद्र द्वारा प्रायोजित।

द्वारा प्रस्तुत         | भूमिका | तारीख
:-------------------- | :--- | :---
Margaret H. Hamilton  | कोलोसस(Colossus) प्रोग्रामिंग लीडर<br>अपोलो गाइडेंस एंड नेविगेशन | 28 Mar 69

के द्वारा अनुमोदित        | भूमिका | तारीख
:----------------- | :--- | :---
Daniel J. Lickly   | निदेशक, मिशन कार्यक्रम विकास<br>अपोलो गाइडेंस एंड नेविगेशन प्रोग्राम | 28 Mar 69
Fred H. Martin     | कोलोसस(Colossus) प्रोजेक्ट मैनेजर<br>अपोलो गाइडेंस एंड नेविगेशन प्रोग्राम | 28 Mar 69
Norman E. Sears    | निदेशक, मिशन विकास<br>अपोलो गाइडेंस एंड नेविगेशन प्रोग्राम | 28 Mar 69
Richard H. Battin  | निदेशक, मिशन विकास<br>अपोलो गाइडेंस एंड नेविगेशन प्रोग्राम | 28 Mar 69
David G. Hoag      | निदेशक<br>अपोलो गाइडेंस एंड नेविगेशन प्रोग्राम | 28 Mar 69
Ralph R. Ragan     | उप निदेशक<br>इंस्ट्रुमेंटेशन प्रयोगशाला | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
