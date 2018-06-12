# Contributing

:crossed_flags:
English,
[한국어][KO_KR]

[EN]:CONTRIBUTING.md
[KO_KR]:CONTRIBUTING.ko_kr.md

## Useful Extensions

GitHub has syntax support for the AGC assembly language built-in.
Unfortunately your code editor will not, however there are
AGC language extensions that provides syntax highlighting for the
following editors:
- [Atom][5]
- [Sublime Text 3][4]
- [Visual Studio Code][3]

## Formatting
**Note:** GitHub and the 3 extensions listed above will ensure
you're using the correct formatting automatically.

- Use tab indentation
- Use tab width of 8
- Trim trailing whitespace

## What do I check?
### Comments
- Comments in the transcribed code should match the scans exactly
  - This could involve creating a deliberate typo or removing/adding
  an entire comment.

### Line breaks
- Line breaks *with* `R0000` in column 1 should match the scans exactly.
- Line breaks *with**__out__* `R0000` in column 1 should contain only 1
or 2 blank lines in a row.
  - If there are more than 2 blank lines breaks, strip the extra
  line breaks.
    - Lines with `R0000` in column 1 do not count towards this.
  - In the source images, these were created by an unprinted digit
  in column 8. A 2 there forced a double space (single blank line)
  and a 3 forced a triple space (double blank line). Values 4-8 were
  defined but never used. Read more about it in [#159][7]

For example the following:
```plain
R0819   SUBROUTINE TO SKIP...
R0820



 0821   LAMPTEST  CS  IMODES33
```
Should become:
```plain
R0819   SUBROUTINE TO SKIP...
R0820


 0820   LAMPTEST  CS  IMODES33
```

## Note

Before you make a PR, please make sure your changes are consistent with the following original AGC printouts!

* [AGC printouts for Comanche][8]
* [AGC printouts for Luminary][9]

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[3]:https://github.com/wopian/agc-assembly
[4]:https://github.com/jimlawton/AGC-Assembly
[5]:https://github.com/Alhadis/language-agc
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
[8]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[9]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
