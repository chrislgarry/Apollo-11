# Contributing
## Guidelines
### Comments
- Comments in the transcribed code should match 1:1 with the
respective image.
  - This could involve creating a deliberate typo or removing/adding
  an entire comment.

### Line breaks
- Line breaks **with** `R0000` in column 1 should match 1:1.
- Line breaks **without** `R0000` in column 1 should only contain 1
to 2 blank lines in a row.
  - If there are more than 2 blank lines breaks, strip the extra
  line breaks.
  - Lines with `R0000` in column 1 do not count towards this number.
  - In the source images, these were created by an unprinted digit
  in column 8. A 2 there forced a double space (single blank line)
  and a 3 forced a triple space (double blank line). Values 4-8 were
  defined but never used. Read more about it in chrislgarry/Apollo-11/#159

## Formatting
GitHub, [AGC Assembly][4] for Sublime Text 3 and [Language AGC][5] for Atom
 will ensure you're using the correct formatting automatically.

[AGC Assembly][3] for Visual Studio Code provides language-specific
settings overrides to add to your user settings [here][6].

- Use tab indentation
- Use tab width of 8
- Trim trailing whitespace

## Useful Extensions

GitHub has syntax support for the AGC assembly language built-in.
Unfortunately your code editor will not. Luckily there is a language
extension that provides syntax highlighting for the following
editors:
- [Visual Studio Code][3]
- [Sublime Text 3][4]
- [Atom][5]

[0]:https://github.com/chrislgarry/Apollo-11/pull/new/master
[1]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[2]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[3]:https://github.com/wopian/agc-assembly
[4]:https://github.com/jimlawton/AGC-Assembly
[5]:https://github.com/Alhadis/language-agc
[6]:https://github.com/wopian/agc-assembly#user-settings
[7]:https://github.com/chrislgarry/Apollo-11/issues/159
