# Apollo-11
[![NASA][1]][2]

:crossed_flags:
[English][EN],
[Español][ES],
Français,
[Português][PT_BR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[EN]:README.md
[ES]:README.es.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md

Code source original de l'ordinateur de guidage Apollo 11 (AGC) pour le module de commande (Comanche055) et le module lunaire (Luminary099). Numérisé par les gens du [Virtual AGC][3] et du [MIT Museum][4]. Le but est d'être un repo pour le code source original d'Apollo 11. En tant que tel, les PR sont les bienvenus pour tous les problèmes identifiés entre les transcriptions dans ce référentiel et les scans source originaux pour [Luminary 099][5] et [Comanche 055][6], ainsi que pour tous les fichiers que j'ai pu manquer.

## Contribuer
Merci de lire [CONTRIBUTING.md][7] avant d' ouvrir un pull request

## Compilation
Si vous êtes interessé par la compilation du code source original visitez [Virtual AGC][8].

## Attribution
```plain
Copyright: Domaine publique.
Nom du fichier:  CONTRACT_AND_APPROVALS.agc
Objet: Partie du code source de Colossus 2A, AKA Comanche 055.
		Partie du code source pour le module de commande (CM)
		Ordinateur embarqué de navigation (AGC), pour Apollo 11.

Assembleur: yaYUL
Contact:   Ron Burkey <info@sandroid.org>.
Site web:   www.ibiblio.org/apollo.
Historique des modifications:   2009-05-06 RSB  Transcriptions à partir d'images de pages.

Ce code source a été transcrit ou autrement adapté à partir de données numérisées.
des images d'une copie papier du Musée du MIT.  La numérisation a été effectuée.
par Paul Fjeld, et organisé par Deborah Douglas du Musée.  Grand merci aux deux.
Les images (avec une réduction appropriée de la taille de stockage et de l'espace de stockage.
de la qualité de l'image) sont disponibles en ligne à l'adresse suivante
www.ibiblio.org/apollo.  Si, pour une raison quelconque, vous constatez que les images sont
illisible, contactez-moi à l'adresse info@sandroid.org pour obtenir l'accès à
(beaucoup) d'images de meilleure qualité que Paul a réellement créées.

Notations sur le document papier lues, en partie :

Assembleur  révision 055 du programme AGC Comanche de la NASA.
2021113-051.  10:28 APR. 1, 1969

Page 1

#************************************************************************
#                                                                       *
#     CE PROGRAMME DE L'AGC SERA ÉGALEMENT DÉSIGNÉ SOUS LE NOM DE:      *
#                                                                       *
#                                                                       *
#               COLOSSUS 2A                                             *
#                                                                       *
#                                                                       *
#   CE PROGRAMME EST DESTINÉ À ÊTRE UTILISÉ EN CM COMME INDIQUÉ         *
#   DANS LE RAPPORT R-577.  CE PROGRAMME A ÉTÉ PRÉPARÉ DANS LE CADRE    *
#   DU DSRDSR PROJET 55-23870, PARRAINÉ PAR L'ENGIN SPATIAL HABITÉ      *
#   CENTRE NATIONAL DE L' AÉRONAUTIQUE ET DE L''ADMINISTRSTION SPATIALE *
#   PAR LE BIAIS DU CONTRAT NAS 9-4065 AVEC LE LABORATOIRE              *
#   D'INSTRUMENTATION,NSTITUT DU MASSACHUSETTS TECHNOLOGIE,             *
#   CAMBRIDGE, MASSE.                                                   *
#************************************************************************


SOUMIS:  MARGARET H. HAMILTON        DATE:   28 MAR 69
    M.H.HAMILTON, COLOSSUS PROGRAMMING LEADER
    APOLLO GUIDE ET NAVIGATION

APPROUVÉ:   DANIEL J. LICKLY        DATE:   28 MAR 69
    D.J.LICKLY, DIRECTOR, MISSION PROGRAM DEVELOPMENT
    APOLLO GUIDANCE AND NAVIGATION PROGRAM

APPROUVÉ:   FRED H. MARTIN          DATE:   28 MAR 69
    FRED H. MARTIN, COLOSSUS PROJECT MANAGER
    APOLLO GUIDE ET PROGRAMME DE NAVIGATION

APPROUVÉ:   NORMAN E. SEARS         DATE:   28 MAR 69
    N.E. SEARS, DIRECTOR, MISSION DEVELOPMENT
    APOLLO GUIDE ET PROGRAMME DE NAVIGATION

APPROUVÉ:   RICHARD H. BATTIN       DATE:   28 MAR 69
    R.H. BATTIN, DIRECTOR, MISSION DEVELOPMENT
    APOLLO GUIDE ET PROGRAMME DE NAVIGATION

APPROUVÉ:   DAVID G. HOAG           DATE:   28 MAR 69
    D.G. HOAG, DIRECTOR
    APOLLO GUIDE ET PROGRAMME DE NAVIGATION

APPROUVÉ:   RALPH R. RAGAN          DATE:   28 MAR 69
    R.R. RAGAN, DEPUTY DIRECTOR
    LABORATOIRE D'INSTRUMENTATION
```

[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc
