# Apollo-11
[![NASA][1]][2]

:crossed_flags:
[Bahasa Indonesia][ID],
[Català][CA],
[Deutsch][DE],
**English**,
[Español][ES],
[Français][FR],
[Italiano][IT],
[Português][PT_BR],
[Русский][RU],
[Türkçe][TR],
[Română][RO],
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
[TR]:README.tr.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md
[HI_IN]:README.hi_in.md
[RU]:README.ru.md
[RO]:README.ro.md

Original Apollo 11 guidance computer (AGC) source code for Command
Module (Comanche055) and Lunar Module (Luminary099). Digitized by
the folks at [Virtual AGC][3] and [MIT Museum][4]. The goal is to be
a repo for the original Apollo 11 source code. As such, PRs are
welcome for any issues identified between the transcriptions in this
repository and the original source scans for [Luminary 099][5] and
[Comanche 055][6], as well as any files I may have missed.

## Contributing
Please read [CONTRIBUTING.md][7] before opening a pull request.

## Compiling
If you are interested in compiling the original source code, check
out [Virtual AGC][8].

## Attribution

&nbsp;         | &nbsp;
:------------- | :-----
Copyright      | Public domain
Comanche055    | Part of the source code for Colossus 2A, the Command Module's (CM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 055 of AGC program Comanche by NASA`<br>`2021113-051. 10:28 APR. 1, 1969`
Luminary099    | Part of the source code for Luminary 1A, the Lunar Module's (LM) Apollo Guidance Computer (AGC) for Apollo 11<br>`Assemble revision 001 of AGC program LYM99 by NASA`<br>`2021112-061. 16:27 JUL. 14, 1969`
Assembler      | yaYUL
Contact        | Ron Burkey <info@sandroid.org>
Website        | www.ibiblio.org/apollo
Digitalization | This source code has been transcribed or otherwise adapted from digitized images of a hardcopy from the MIT Museum. The digitization was performed by Paul Fjeld, and arranged for by Deborah Douglas of the Museum. Many thanks to both.

### Contract and Approvals
*Derived from [CONTRACT_AND_APPROVALS.agc]*

This AGC program shall also be referred to as Colossus 2A.

This program is intended for use in the CM as specified in report `R-577`. This program was prepared under DSR project `55-23870`, sponsored by the Manned Spacecraft Center of The National Aeronautics and Space Administration through contract `NAS 9-4065` with the Instrumentation Laboratory, Massachusetts Institute of Technology, Cambridge, Mass.

Submitted by          | Role | Date
:-------------------- | :--- | :---
Margaret H. Hamilton  | Colossus Programming Leader<br>Apollo Guidance and Navigation | 28 Mar 69

Approved by        | Role | Date
:----------------- | :--- | :---
Daniel J. Lickly   | Director, Mission Program Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Fred H. Martin     | Colossus Project Manager<br>Apollo Guidance and Navigation Program | 28 Mar 69
Norman E. Sears    | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
Richard H. Battin  | Director, Mission Development<br>Apollo Guidance and Navigation Program | 28 Mar 69
David G. Hoag      | Director<br>Apollo Guidance and Navigation Program | 28 Mar 69
Ralph R. Ragan     | Deputy Director<br>Instrumentation Laboratory | 28 Mar 69

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[1]:https://rawcdn.githack.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc

    Apache License
                           Version 2.0, January 2004
                        https://www.apache.org/licenses/

   TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

   1. Definitions.

      "License" shall mean the terms and conditions for use, reproduction,
      and distribution as defined by Sections 1 through 9 of this document.

      "Licensor" shall mean the copyright owner or entity authorized by
      the copyright owner that is granting the License.

      "Legal Entity" shall mean the union of the acting entity and all
      other entities that control, are controlled by, or are under common
      control with that entity. For the purposes of this definition,
      "control" means (i) the power, direct or indirect, to cause the
      direction or management of such entity, whether by contract or
      otherwise, or (ii) ownership of fifty percent (50%) or more of the
      outstanding shares, or (iii) beneficial ownership of such entity.

      "You" (or "Your") shall mean an individual or Legal Entity
      exercising permissions granted by this License.

      "Source" form shall mean the preferred form for making modifications,
      including but not limited to software source code, documentation
      source, and configuration files.

      "Object" form shall mean any form resulting from mechanical
      transformation or translation of a Source form, including but
      not limited to compiled object code, generated documentation,
      and conversions to other media types.

      "Work" shall mean the work of authorship, whether in Source or
      Object form, made available under the License, as indicated by a
      copyright notice that is included in or attached to the work
      (an example is provided in the Appendix below).

      "Derivative Works" shall mean any work, whether in Source or Object
      form, that is based on (or derived from) the Work and for which the
      editorial revisions, annotations, elaborations, or other modifications
      represent, as a whole, an original work of authorship. For the purposes
      of this License, Derivative Works shall not include works that remain
      separable from, or merely link (or bind by name) to the interfaces of,
      the Work and Derivative Works thereof.

      "Contribution" shall mean any work of authorship, including
      the original version of the Work and any modifications or additions
      to that Work or Derivative Works thereof, that is intentionally
      submitted to Licensor for inclusion in the Work by the copyright owner
      or by an individual or Legal Entity authorized to submit on behalf of
      the copyright owner. For the purposes of this definition, "submitted"
      means any form of electronic, verbal, or written communication sent
      to the Licensor or its representatives, including but not limited to
      communication on electronic mailing lists, source code control systems,
      and issue tracking systems that are managed by, or on behalf of, the
      Licensor for the purpose of discussing and improving the Work, but
      excluding communication that is conspicuously marked or otherwise
      designated in writing by the copyright owner as "Not a Contribution."

      "Contributor" shall mean Licensor and any individual or Legal Entity
      on behalf of whom a Contribution has been received by Licensor and
      subsequently incorporated within the Work.

   2. Grant of Copyright License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      copyright license to reproduce, prepare Derivative Works of,
      publicly display, publicly perform, sublicense, and distribute the
      Work and such Derivative Works in Source or Object form.

   3. Grant of Patent License. Subject to the terms and conditions of
      this License, each Contributor hereby grants to You a perpetual,
      worldwide, non-exclusive, no-charge, royalty-free, irrevocable
      (except as stated in this section) patent license to make, have made,
      use, offer to sell, sell, import, and otherwise transfer the Work,
      where such license applies only to those patent claims licensable
      by such Contributor that are necessarily infringed by their
      Contribution(s) alone or by combination of their Contribution(s)
      with the Work to which such Contribution(s) was submitted. If You
      institute patent litigation against any entity (including a
      cross-claim or counterclaim in a lawsuit) alleging that the Work
      or a Contribution incorporated within the Work constitutes direct
      or contributory patent infringement, then any patent licenses
      granted to You under this License for that Work shall terminate
      as of the date such litigation is filed.

   4. Redistribution. You may reproduce and distribute copies of the
      Work or Derivative Works thereof in any medium, with or without
      modifications, and in Source or Object form, provided that You
      meet the following conditions:

      (a) You must give any other recipients of the Work or
          Derivative Works a copy of this License; and

      (b) You must cause any modified files to carry prominent notices
          stating that You changed the files; and

      (c) You must retain, in the Source form of any Derivative Works
          that You distribute, all copyright, patent, trademark, and
          attribution notices from the Source form of the Work,
          excluding those notices that do not pertain to any part of
          the Derivative Works; and

      (d) If the Work includes a "NOTICE" text file as part of its
          distribution, then any Derivative Works that You distribute must
          include a readable copy of the attribution notices contained
          within such NOTICE file, excluding those notices that do not
          pertain to any part of the Derivative Works, in at least one
          of the following places: within a NOTICE text file distributed
          as part of the Derivative Works; within the Source form or
          documentation, if provided along with the Derivative Works; or,
          within a display generated by the Derivative Works, if and
          wherever such third-party notices normally appear. The contents
          of the NOTICE file are for informational purposes only and
          do not modify the License. You may add Your own attribution
          notices within Derivative Works that You distribute, alongside
          or as an addendum to the NOTICE text from the Work, provided
          that such additional attribution notices cannot be construed
          as modifying the License.

      You may add Your own copyright statement to Your modifications and
      may provide additional or different license terms and conditions
      for use, reproduction, or distribution of Your modifications, or
      for any such Derivative Works as a whole, provided Your use,
      reproduction, and distribution of the Work otherwise complies with
      the conditions stated in this License.

   5. Submission of Contributions. Unless You explicitly state otherwise,
      any Contribution intentionally submitted for inclusion in the Work
      by You to the Licensor shall be under the terms and conditions of
      this License, without any additional terms or conditions.
      Notwithstanding the above, nothing herein shall supersede or modify
      the terms of any separate license agreement you may have executed
      with Licensor regarding such Contributions.

   6. Trademarks. This License does not grant permission to use the trade
      names, trademarks, service marks, or product names of the Licensor,
      except as required for reasonable and customary use in describing the
      origin of the Work and reproducing the content of the NOTICE file.

   7. Disclaimer of Warranty. Unless required by applicable law or
      agreed to in writing, Licensor provides the Work (and each
      Contributor provides its Contributions) on an "AS IS" BASIS,
      WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or
      implied, including, without limitation, any warranties or conditions
      of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A
      PARTICULAR PURPOSE. You are solely responsible for determining the
      appropriateness of using or redistributing the Work and assume any
      risks associated with Your exercise of permissions under this License.

   8. Limitation of Liability. In no event and under no legal theory,
      whether in tort (including negligence), contract, or otherwise,
      unless required by applicable law (such as deliberate and grossly
      negligent acts) or agreed to in writing, shall any Contributor be
      liable to You for damages, including any direct, indirect, special,
      incidental, or consequential damages of any character arising as a
      result of this License or out of the use or inability to use the
      Work (including but not limited to damages for loss of goodwill,
      work stoppage, computer failure or malfunction, or any and all
      other commercial damages or losses), even if such Contributor
      has been advised of the possibility of such damages.

   9. Accepting Warranty or Additional Liability. While redistributing
      the Work or Derivative Works thereof, You may choose to offer,
      and charge a fee for, acceptance of support, warranty, indemnity,
      or other liability obligations and/or rights consistent with this
      License. However, in accepting such obligations, You may act only
      on Your own behalf and on Your sole responsibility, not on behalf
      of any other Contributor, and only if You agree to indemnify,
      defend, and hold each Contributor harmless for any liability
      incurred by, or claims asserted against, such Contributor by reason
      of your accepting any such warranty or additional liability.

   END OF TERMS AND CONDITIONS

   APPENDIX: How to apply the Apache License to your work.

      To apply the Apache License to your work, attach the following
      boilerplate notice, with the fields enclosed by brackets "[]"
      replaced with your own identifying information. (Don't include
      the brackets!)  The text should be enclosed in the appropriate
      comment syntax for the file format. We also recommend that a
      file or class name and description of purpose be included on the
      same "printed page" as the copyright notice for easier
      identification within third-party archives.

   Copyright 2019 Rolando Gopez Lacuata.

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       https://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
