# Contribuer

🎌
[Čeština][CZ],
[Deutsch][DE],
[English][EN],
[Español][ES],
**Français**,
[Kurdi][KU],
[Nederlands][NL],
[Português][PT_BR],
[Türkçe][TR],
[العربية][AR],
[Ελληνικά][GR],
[日本語][JA],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
[한국어][KO_KR]

[AR]:CONTRIBUTING.ar.md
[CZ]:CONTRIBUTING.cz.md
[DE]:CONTRIBUTING.de.md
[EN]:CONTRIBUTING.md
[ES]:CONTRIBUTING.es.md
[FR]:CONTRIBUTING.fr.md
[GR]:CONTRIBUTING.gr.md
[JA]:CONTRIBUTING.ja.md
[KO_KR]:CONTRIBUTING.ko_kr.md
[KU]:CONTRIBUTING.ku.md
[NL]:CONTRIBUTING.nl.md
[PT_BR]:CONTRIBUTING.pt_br.md
[TR]:CONTRIBUTING.tr.md
[ZH_CN]:CONTRIBUTING.zh_cn.md
[ZH_TW]:CONTRIBUTING.zh_tw.md

Le code source de ce dépôt a été numérisé manuellement à partir d’imprimés papier, de sorte que les fautes de frappe et autres anomalies ont été introduites accidentellement. Le code doit être modifié pour être cohérent avec les impressions numérisées:

- [Impressions AGC pour Comanche][8]
- [Impressions AGC pour Luminary][9]

## Extensions utiles

Github prend en charge nativement la syntaxe pour le langage assembleur AGC. Malheureusement, votre éditeur de texte ne l’aura pas. Mais il y a des extensions pour le langage AGC pour les éditeurs suivants:

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

**Note:** GitHub et les extensions marquées ci-dessus vous assureront d'utiliser automatiquement le bon formatage.

- Indenter avec tabulation
- Les tabulations ont une taille de 8
- Pas d'espace à la fin des ligne

## Comment vérifier ?

Tout écart entre les scans et le code source dans ce référentiel, y compris :

### Commentaires

- Les commentaires dans le code transcrit doivent correspondre exactement aux scans
  - Cela peut impliquer de créer délibérément une erreur de frappe  ou de supprimer/ajouter un commentaire entier.

### Sauts de ligne

- Les lignes *avec* `R0000` dans la colonne 1 doivent correspondre exactement aux scans.
- Les sauts de ligne *sans* `R0000` dans la colonne 1 ne doivent contenir que 1 ou 2 lignes vides d'affilée.
  - Si il y a plus de 2 lignes vides, supprimer les sauts de ligne supplémentaires.
    - Ne pas prendre en compte les lignes avecc `R0000` dans la cononne 1.
  - Dans les images sources, il y a digits non imprimés dans la colonne 8. Un 2 force un double espace (une seul ligne vide) et un 3 force une triple espace (double ligne vide). Les valeurs 4-8 ont été définies mais n’ont jamais été utilisées. Pour en savoir plus [#159][7]

Par exemple, ce qui suit:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Doit devenir:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Espaces

- Les espaces entre deux caractères doivent respecter la convention suivante (voir la discussion [#316][10]):
  - Un seul espace pour un nouveau mot.
  - Deux espaces pour une nouvelle phrase.
  - Trois espaces pour l'indentations.

Par exemple, ce qui suit:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Doit devenir:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Note

Avant de faire une PR, assurez-vous que vos modifications sont cohérentes avec les scans!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
