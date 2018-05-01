# Apollo-11
[![NASA][1]][2]

:crossed_flags:
[English][EN],
[Español][ES],
[Français][FR],
[Português][PT_BR],
[正體中文][ZH_TW],
[简体中文][ZH_CN],
한국어

[EN]:README.md
[ES]:README.es.md
[FR]:README.fr.md
[PT_BR]:README.pt_br.md
[ZH_TW]:README.zh_tw.md
[ZH_CN]:README.zh_cn.md
[KO_KR]:README.ko_kr.md

사령선 모듈 (Comanche055) 및 달 착륙선 모듈 (Luminary099)을 위한 아폴로 11호 유도 컴퓨터(AGC)의 소스코드입니다. [Virtual AGC][3] 및 [MIT Museum][4]에 의해 디지털화 되었습니다.
이 레포지토리는 아폴로 11호의 원본 소스코드를 완벽하게 재현하여 보관하는 것을 목표로 합니다. 따라서 원본과 본 디지털본 간에 발견된 모든 이슈, 또한 빠뜨렸을 듯한 파일들에 대해 PR을 환영합니다.

## 기여하기
PR을 열기 전에 [CONTRIBUTING.md][7] 을 읽어보시기 바랍니다.

## 컴파일
만약 컴파일을 원하신다면 [Virtual AGC][8] 을 확인하여 보십시오.

## 권한
```plain
저작권: 퍼블릭 도메인
파일명: CONTRACT_AND_APPROVALS.agc
목적  : 콜로서스 2A(Comanche 055)의 소스코드 중 일부.
      아폴로 11호의 AGC 통제 모듈 소스코드의 일부.
어셈블러: yaYUL
연락처 : Ron Burkey <info@sandroid.org>
웹사이트 : www.ibiblio.org/apollo
수정 기록 : 2009-05-06 RSB  문서 이미지에서 옮겨짐

이 소스코드는 MIT Museum에서 하드카피본을 디지털화, 옮긴 것 입니다.
디지털화는 Paul Fjeld에 의해 이루어졌으며, Deborah Douglas가 정리하였습니다.
두 분께 깊은 감사를 표합니다.
이미지는 www.ibiblio.org/apollo 에서 보실 수 있습니다. 만일 해당 이미지를 판독하기
어렵다면 info@sandroid.org로 연락을 주시기 바랍니다.
Paul씨가 실제로 만들어 낸 이미지에 접근하실 수 있도록 도와드리겠습니다.

다음은 하드카피 문서의 일부분입니다 :

AGC 프로그램 Comanche 어셈블 판본 055, NASA에 의해 작성됨.
2021113-051.  10:28 1969년 4월 1일

1 페이지

#************************************************************************
#
#       본 아폴로 유도 컴퓨터 프로그램은 다음에 의해 참조됩니다 :
#
#
#               콜로서스 2A
#
#
#   이 프로그램은 R-577에 명세된 사령선 모듈에 사용됩니다.            
#   DSR 프로젝트 55-23870 에 의해 준비되었으며, NASA 유인 우주선 센터가               
#   MIT 기계 연구소 와의 NAS 9-4065 계약에 의해 스폰싱 하였습니다.
#
#************************************************************************


제출됨: MARGARET H. HAMILTON           일시:    1969년 3월 28일
    M.H.HAMILTON, 콜로서스 프로그래밍 리더
    아폴로 유도 및 항해

승인됨: DANIEL J. LICKLY    일시: 1969년 3월 28일
    D.J.LICKLY, 감독, 임무 프로그램 개발
    아폴로 유도 및 항법 프로그램

승인됨: FRED H. MARTIN      일시: 1969년 3월 28일
    FRED H. MARTIN, 콜로서스 프로젝트 매니저
    아폴로 유도 및 항법 프로그램

승인됨: NORMAN E. SEARS      일시: 1969년 3월 28일
    N.E. SEARS, 감독, 임무 개발
    아폴로 유도 및 항법 프로그램

승인됨: FRED H. MARTIN      일시: 1969년 3월 28일
    R.H. BATTIN, 감독, 임무 개발
    아폴로 유도 및 항법 프로그램

승인됨: DAVID G. HOAG      일시: 1969년 3월 28일
    D.G. HOAG, 감독
    아폴로 유도 및 항법 프로그램

승인됨: FRED H. MARTIN      일시: 1969년 3월 28일
    R.R. RAGAN, 부감독
    기계 연구소
```

[1]:https://cdn.rawgit.com/aleen42/badges/c9246f74/src/nasa.svg
[2]:https://www.nasa.gov/mission_pages/apollo/missions/apollo11.html
[3]:http://www.ibiblio.org/apollo/
[4]:http://web.mit.edu/museum/
[5]:http://www.ibiblio.org/apollo/ScansForConversion/Luminary099/
[6]:http://www.ibiblio.org/apollo/ScansForConversion/Comanche055/
[7]:https://github.com/chrislgarry/Apollo-11/blob/master/CONTRIBUTING.md
[8]:https://github.com/rburkey2005/virtualagc

