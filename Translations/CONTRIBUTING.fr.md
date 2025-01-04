# Contribuer

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

Le code source de ce dépôt a été numérisé manuellement à partir de papiers imprimés, les fautes de frappe et autres anomalies ont donc été introduites accidentellement. Le code doit être modifié pour être cohérent avec les impressions numérisées suivantes :

- [Impressions AGC pour Comanche][8]
- [Impressions AGC pour Luminary][9]

Le site Web suivant peut être utilisé pour naviguer facilement dans les impressions numérisées de Comanche et Luminary : https://28gpc.csb.app/

## Extensions utiles

GitHub prend en charge nativement la syntaxe pour le langage assembleur AGC. Malheureusement, votre éditeur de texte ne l’aura pas. Mais il y a des extensions pour le langage AGC pour les éditeurs suivants :

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

† Prend en charge le formatage automatique

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

## Mise en page

**Note :** GitHub et les extensions marquées ci-dessus vous assureront d'utiliser automatiquement le bon formatage.

- Utiliser une tabulation (tab) pour l'indentation
- Utiliser une largeur de 8 caractères pour la tabulation
- Pas d'espace à la fin des lignes

## Comment vérifier ?

Tout écart entre les scans et le code source dans ce référentiel.

### Commentaires

- Les commentaires dans le code transcrit **doivent** correspondre **exactement** aux scans.
  - Les problèmes courants que vous devez rechercher lors de la vérification sont les suivants. Attention, la liste n'est pas exhaustive !

#### Erreurs typographiques

À certains endroits, les développeurs originaux ont fait des erreurs typographiques en écrivant des commentaires. Certaines d’entre elles ont été corrigées par erreur lors de la numérisation initiale, mais la numérisation a également introduit des erreurs typographiques qui n’étaient pas présentes dans les scans.

Par exemple, si les commentaires numérisés contiennent `SPACECRAFT`, mais que `SPAECRAFT` a été imprimé dans les scans, la numérisation DOIT être corrigée en `SPAECRAFT` (C manquant).

De même, si un mot a une faute de frappe dans la numérisation mais est correctement orthographié dans les scans, alors la faute de frappe DOIT être corrigée.

#### Espaces

Les espaces entre deux caractères dans les commentaires **DEVRAIENT** correspondre aux scans. Dans la plupart des cas (voir la discussion dans [#316][10]), c'est:

- Espace unique pour les nouveaux mots
- Double espace pour les nouvelles phrases
- Triple espace pour les indentations

Toutes les pages des scans ne suivent pas cette généralisation, si les scans n'ont qu'un seul espace au lieu d'un double espace, utiliser un seul espace.

### Sauts de ligne

- Les lignes *avec* `R0000` dans la colonne 1 doivent correspondre exactement aux scans.
- Les sauts de ligne *sans* `R0000` dans la colonne 1 ne doivent contenir que 1 ou 2 lignes vides d'affilée.
  - Si il y a plus de 2 lignes vides, supprimer les sauts de ligne supplémentaires.
    - Ne pas prendre en compte les lignes avec `R0000` dans la colonne 1.
  - Dans les images sources, celles-ci ont été créées à cause d'un caractère non imprimé dans la colonne 8. Un 2 a forcé un double espace (une seule ligne vide) et un 3 a forcé un triple espace (double ligne vide). Les valeurs 4-8 ont été définies mais n’ont jamais été utilisées. Pour en savoir plus [#159][7]

Par exemple, ce qui suit :

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Doit devenir :

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Note

Avant de faire une PR, assurez-vous que vos modifications soient cohérentes avec les scans !

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
