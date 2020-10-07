# Συνεισφορά

🎌
[Čeština][CZ],
[Deutsch][DE],
[English][EN],
[Español][ES],
[Français][FR],
[Kurdi][KU],
[Nederlands][NL],
[Português][PT_BR],
[Türkçe][TR],
[العربية][AR],
**Ελληνικά**,
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

Ο πηγαίος κώδικας σε αυτό το αποθετήριο ψηφιοποιήθηκε χειροκίνητα (με μη αυτόματο τρόπο) από εκτυπώσεις σε χαρτί, έτσι τυχαία λάθη και άλλες αποκλίσεις μπορεί να έχουν εισαχθεί κατά λάθος. Ο κώδικας πρέπει να τροποποιείται ώστε πάντα να είναι συνεπής με τις σαρωμένες εκτυπώσεις:

- [AGC printouts for Comanche][8]
- [AGC printouts for Luminary][9]

## Χρήσιμες Επεκτάσεις

Το GitHub διαθέτει υποστήριξη συντακτικού για τη γλώσσα assembly του AGC. Δυστυχώς, ο επεξεργαστής κώδικά σας μπορεί να μην την υποστηρίζει, ωστόσο, υπάρχουν επεκτάσεις για τη γλώσσα assembly AGC που παρέχουν επισήμανση σύνταξης για τους ακόλουθους επεξεργαστές κώδικα:

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

† Υποστηρίζει αυτόματη μορφοποίηση

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

## Mορφοποίηση

Σημείωση: Το GitHub και οι επεκτάσεις που σημειώνονται παραπάνω θα διασφαλίσουν ότι χρησιμοποιείτε τη σωστή μορφοποίηση αυτόματα.

- Χρήση εσοχής με tab
- 8 χαρακτήες κενού για εσοχή tab
- Περικοπή τελικών κενών χαρακτήρων γραμμής

## Τί να ελέγξω;

Τυχόν αποκλίσεις μεταξύ των σαρώσεων και του πηγαίου κώδικα σε αυτό το αποθετήριο, συμπεριλαμβανομένων:

### Σχόλια

- Τα σχόλια στον μεταγραμμένο κώδικα πρέπει να ταιριάζουν ακριβώς με τις σαρώσεις
  - Αυτό μπορεί να περιλαμβάνει τη δημιουργία σκόπιμου τυπογραφικού λάθους ή την κατάργηση / προσθήκη ολόκληρου σχολίου.

### Αλλαγές γραμμής

- Οι αλλαγές γραμμής με `R0000` στη στήλη 1 πρέπει να ταιριάζουν ακριβώς με τις σαρώσεις.
- Οι αλλαγές γραμμής *χωρίς* `R0000` στη στήλη 1 πρέπει να περιέχουν μόνο 1 ή 2 κενές γραμμές στη σειρά.
  - Εάν υπάρχουν περισσότερες από 2 αλλαγές γραμμών, αφαιρέστε τις επιπλέον αλλαγές γραμμής.
    - Οι γραμμές με `R0000` στη στήλη 1 δεν υπολογίζονται σε αυτό.
  -  Στις εικόνες προέλευσης, αυτές δημιουργήθηκαν από ένα μη τυπωμένο ψηφίο στη στήλη 8. Ένα 2 ανάγκαζε ένα διπλό διάστημα (μονή κενή γραμμή) και ένα 3 ανάγκαζε ένα τριπλό διάστημα (διπλή κενή γραμμή). Οι τιμές 4-8 ορίστηκαν αλλά δεν χρησιμοποιήθηκαν ποτέ. Διαβάστε περισσότερα σχετικά, στο [#159][7]

Για παράδειγμα το παρακάτω:

```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```

Πρέπει να γίνει:

```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

### Κενά

- Τα διαστήματα μεταξύ δύο χαρακτήρων στη συμβολοσειρά πρέπει να τηρούν την ακόλουθη σύμβαση (δείτε τη συζήτηση στο [#316][10]):
  - Ένας κενός χαρακτήρας για νέες λέξεις.
  - Δύο κενοί χαρακτήρες για νέες προτάσεις.
  - Τρεις κενοί χαρακτήες για εσοχές.

Για παράδειγμα το παρακάτω:

```plain
	1)  FOO BAR BAZ QUX QUUX QUUZ. CORGE, GRAULT,
	GARPLY, WALDO.
```

Πρέπει να γίνει:

```plain
	1) FOO BAR BAZ QUX QUUX QUUZ.  CORGE, GRAULT,
	   GARPLY, WALDO.
```

## Σημείωση

Πριν κάνετε ένα PR, παρακαλώ βεβαιωθείτε ότι οι αλλαγές σας είναι σύμφωνες με τις σαρώσεις!

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[10]:https://github.com/chrislgarry/Apollo-11/pull/316#pullrequestreview-102892741
