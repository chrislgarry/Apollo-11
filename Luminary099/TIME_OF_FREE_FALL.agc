# Copyright:	Public domain.
# Filename:	TIME_OF_FREE_FALL.agc
# Purpose: 	Part of the source code for Luminary 1A build 099.
#		It is part of the source code for the Lunar Module's (LM)
#		Apollo Guidance Computer (AGC), for Apollo 11.
# Assembler:	yaYUL
# Contact:	Ron Burkey <info@sandroid.org>.
# Website:	www.ibiblio.org/apollo.
# Pages:	1268-1283
# Mod history:	2009-05-26 RSB	Adapted from the corresponding
#				Luminary131 file, using page
#				images from Luminary 1A.
#		2011-01-06 JL	Fixed pseudo-label indentation.
#
# This source code has been transcribed or otherwise adapted from
# digitized images of a hardcopy from the MIT Museum.  The digitization
# was performed by Paul Fjeld, and arranged for by Deborah Douglas of
# the Museum.  Many thanks to both.  The images (with suitable reduction
# in storage size and consequent reduction in image quality as well) are
# available online at www.ibiblio.org/apollo.  If for some reason you
# find that the images are illegible, contact me at info@sandroid.org
# about getting access to the (much) higher-quality images which Paul
# actually created.
#
# Notations on the hardcopy document read, in part:
#
#	Assemble revision 001 of AGC program LMY99 by NASA 2021112-061
#	16:27 JULY 14, 1969

# Page 1268
#          THE TFF SUBROUTINES MAY BE USED IN EITHER EARTH OR MOON CENTERED COORDINATES. THE TFF ROUTINES NEVER
# KNOW WHICH ORIGIN APPLIES. IT IS THE USER WHO KNOWS, AND WHO SUPPLIES  RONE, VONE, AND 1/SQRT(MU) AT THE
# APPROPRIATE SCALE LEVEL FOR THE PROPER PRIMARY BODY.
#
#          EARTH ORIGIN           POSITION      -29       METERS
#                                 VELOCITY      -7        METERS/CENTISECOND
#                                 1/SQRT(MU)    +17       SQRT(CS SQ/METERS CUBED)
#
#          MOON ORIGIN            POSITION      -27       METERS
#                                 VELOCITY      -5        METERS/CENTISECONDS
#                                 1/SQRT(MU)    +14       SQRT(CS SQ/METERS CUBED)
#
# ALL DATA PROVIDED TO AND RECEIVED FROM ANY TFF SUBROUTINE WILL BE AT ONE OF THE LEVELS ABOVE. IN ALL CASES,
# THE FREE FALL TIME IS RETURNED IN CENTISECONDS AT (-28). PROGRAM  TFF/CONIC  WILL GENERATE VONE/RTMU  AND
# LEAVE IT IN  VONE' AT (+10) IF EARTH ORIGIN AND (+9) IF MOON ORIGIN.
#          THE USER MUST STORE THE STATE VECTOR IN  RONE, VONE  AND MU IN THE FORM 1/SQRT(MU) IN TFF/RTMU
# AT THE PROPER SCALE BEFORE CALLING  TFF/CONIC. SINCE RONE, VONE   ARE IN THE EXTENDED VERB STORAGE AREA,
# THE USER MUST ALSO LOCK OUT THE EXTENDED VERBS, AND RELEASE THEM WHEN FINISHED.
#          PROGRAMS     CALC/TFF  AND  CALC/TPER  ASSUME THAT THE TERMINAL RADIUS IS LESS THAN THE PRESENT
# RADIUS. THIS RESTRICTION CAN BE REMOVED BY A  15 W CODING CHANGE, BUT AT PRESENT IT IS NOT DEEMED NECESSARY.
#
#          THE FOLLOWING ERASABLE QUANTITIES ARE USED BY THE TFF ROUTINES, AND ARE LOCATED IN THE PUSH LIST.
#
#
#                                                              BELOW      E:   IS USED FOR EARTH ORIGIN SCALE
#                                                                         M:   IS USED FOR MOON  ORIGIN SCALE
#
#TFFSW		=	119D	#                               BIT1      0 = CALCTFF             1 = CALCTPER
TFFDELQ		=	10D	#                                         Q2-Q1                   E: (-16)  M: (-15)
RMAG1		=	12D	#                                         ABVAL(RN)  M            E: (-29)  M: (-27)
#RPER		=	14D	#                                         PERIGEE RADIUS  M       E: (-29)  M: (-27)
TFFQ1		=	14D	#                                         R.V / SQRT(MUE)         E: (-16)  M: (-15)
#SDELF/2			#                                         SIN(THETA) /2
CDELF/2		=	14D	#                                         COS(THETA) /2
#RAPO		=	16D	#                                         APOGEE RADIUS  M        E: (-29)  M: (-27)
NRTERM		=	16D	#                                         TERMINAL RADIUS  M      E: (-29+NR)
				#                                                                 M: (-27+NR)
RTERM		=	18D	#                                         TERMINAL RADIUS  M      E: (-29)  M: (-27)
TFFVSQ		=	20D	#                                         -(V SQUARED/MU)  1/M    E: (20)   M: (18)
TFF1/ALF	=	22D	#                                         SEMI MAJ AXIS  M        E: (-22-2 NA)
				#                                                                 M: (-20-2 NA)
TFFRTALF	=	24D	#                                         SQRT(ALFA)              E:(10+NA) M: (9+NA)
TFFALFA		=	26D	#                                         ALFA  1/M               E:(26-NR) M: (24-NR)
TFFNP		=	28D	#                                         SEMI LATUS RECTUM  M    E: (-38+2 NR)
				#                                                                 M: (-36+2 NR)
TFF/RTMU	=	30D	#                                         1/SQRT(MU)              E: (17)   M: (14)
NRMAG		=	32D	#                                         PRESENT RADIUS  M       E: (-29+NR)
				#                                                                 M: (-27+NR)
TFFX		=	34D
TFFTEM		=	36D	#                                         TEMPORARY
# Page 1269
#                                          REGISTERS S1, S2 ARE UNTOUCED BY ANY TFF SUBROUTINE
#                                          INDEX REGISTERS X1, X2 ARE USED BY ALL TFF SUBROUTINES. THEY ARE ESTAB-
#		                           LISHED IN TFF/CONIC AND MUST BE PRESERVED BETWEEN CALLS TO SUBSEQUENT
#                                          SUBROUTINES.
#                                          -NR                             C(X1) = NORM COUNT OF RMAG
#                                          -NA                             C(X2)= NORM COUNT OF SQRT(ABS(ALFA))

# Page 1270
# SUBROUTINE NAME:     TFFCONIC                                         DATE:  01.29.67
# MOD NO:  0                                                            LOG SECTION:  TIME OF FREE FALL
# MOD BY:  RR BAIRNSFATHER
# MOD NO:  1           MOD BY:  RR BAIRNSFATHER	 DATE:  11 APR 67
# MOD NO:  2           MOD BY:  RR BAIRNSFATHER	 DATE:  21 NOV 67     ADD MOON MU.
# MOD NO:  3           MOD BY:  RR BAIRNSFATHER	 DATE:  21 MAR 68     ACCEPT DIFFERENT EARTH/MOON SCALES
# FUNCTIONAL DESCRIPTION:      THIS SUBROUTINE IS CALLED TO COMPUTE THOSE CONIC PARAMETERS REQUIRED BY THE TFF
#          SUBROUTINES AND TO ESTABLISH THEM IN THE PUSH LIST AREA. THE PARAMETERS ARE LISTED UNDER OUTPUT.
#          THE EQUATIONS ARE:
#                 _   _  _
#                 H = RN*VN                                               ANGULAR MOMENTUM
#                       _ _
#                 LCP = H.H / MU                                          SEMI LATUS RECTUM
#                               _  _
#                 ALFA = 2/RN - VN.VN / MU                                RECIPROCAL SEMI-MAJOR AXIS, SIGNED
#
#          AND ALFA IS POS FOR ELLIPTIC ORBITS
#                       0 FOR PARABOLIC ORBITS
#                      NEG FOR HYPERBOLIC ORBITS.
#          SUBROUTINE ALSO COMPUTES AND SAVES RMAG.
# CALLING SEQUENCE:
#          TFFCONIC  EXPECTS CALLER TO ENTER WITH CORRECT GRAVITATIONAL CONSTANT IN MPAC, IN THE FORM
#          1/SQRT(MU). PROGRAM WILL SAVE IN TFF/RTMU   . THE SCALE IS DETERMINED BY WHETHER EARTH OR MOON
#          ORIGIN IS USED.  THE CALLER MUST LOCK OUT THE EXTENDED VERBS BEFORE PROVIDING STATE VECTOR IN RONE,
#          VONE  AT PROPER SCALE. THE EXTENDED VERBS MUST BE RESTORED WHEN THE CALLER IS FINISHED USING THE
#          TFF ROUTINES.
#          ENTRY POINT  TFFCONMU  EXPECTS THAT TFF/RTMU IS ALREADY LOADED.
#          TO SPECIFY MU:      DLOAD  CALL              IF MU ALREADY STORED:      CALL
#                                     YOURMU      1/RTMU  E: (17)  M: (14)                TFFCONMU
#                                     TFFCONIC
#          PUSHLOC = PDL+0, ARBITRARY IF LEQ 18D
#
# SUBROUTINES CALLED:  NONE
# NORMAL EXIT MODES:   RVQ
# ALARMS:  NONE
# OUTPUT:  THE FOLLOWING ARE STORED IN THE PUSH LIST AREA.
#          RMAG1  E:(-29) M:(-27) M  RN, PRESENT RADIUS LENGTH.
#          NRMAG  E: (-29+NR)     M  RMAG, NORMALIZED
#                 M: (-27+NR)
#          X1                     -NR, NORM COUNT
#          TFFNP  E: (-38+2NR)    M  LCP, SEMI LATUS RECTUM, WEIGHTED BY NR.       FOR VGAMCALC.
#                 M: (-36+2NR)
#          TFF/RTMU  E:(17)  M:(14)       1/SQRT(MU)
#          TFFVSQ E:(20)  M:(18)  1/M   -(V SQ/MU): PRESENT VELOCITY,NORMALIZED.   FOR VGAMCALC
#          TFFALFA  E: (26-NR)    1/M   ALFA, WEIGHTED BY NR
#                   M: (24-NR)
#          TFFRTALF E: (10+NA)    SQRT(ALFA), NORMALIZED
#                   M: (9+NA)
# Page 1271
#          X2				-NA, NORMCOUNT
#          TFF1/ALF  E:(-22-2NA)	SIGNED SEMI MAJ AXIS, WEIGHTED BY NA
#                    M:(-20-2NA)
#          PUSHLOC AT PDL+0
#          THE FOLLOWING  IS STORED IN GENERAL ERASABLE
#          VONE'  E:(10) M:(9)    V/RT(MU), NORMALIZED VELOCITY
# ERASABLE INITIALIZATION REQUIRED:
#          RONE   E:(-29)  M:(-27)  M  STATE VECTOR		LEFT BY CALLER
#          VONE   E:(-7)   M:(-5)   M/CS  STATE VECTOR          LEFT BY CALLER
#          TFF/RTMU  E:(17)  M:(14)      1/RT(CS SQ/M CUBE)      IF ENTER VIA TFFCONMU.
# DEBRIS:  QPRET     PDL+0 ... PDL+3


		BANK	33
		SETLOC	TOF-FF
		BANK

		COUNT*	$$/TFF

TFFCONIC	STORE	TFF/RTMU	# 1/SQRT(MU)       E:(17) M:(14)

TFFCONMU	VLOAD	UNIT		# COME HERE WITH TFFRTMU LOADED.
			RONE		# SAVED RN.  M     E: (-29)  M: (-27)
		PDDL			# UR/2 TO PDL+0, +5
			36D		# MAGNITUDE
		STORE	RMAG1		# M    E: (-29)   M: (-27)

		NORM
			X1		# -NR
		STOVL	NRMAG		# RMAG  M   E: (-29+NR)  M: (-27+NR)
			VONE		# SAVED VN.  M/CS   E:(-7)    M:(-5)
		VXSC
			TFF/RTMU	# E: (17)   M: (14)
		STORE	VONE'		# VN/SQRT(MU)   E:(10)   M:(9)

		VXSC	VXV
			NRMAG		# E: (-29+NR)  M: (-27+NR)
					# UR/2 FROM PDL
		VSL1	VSQ		# BEFORE:  E:(-19+NR) M:(-18+NR)
		STODL	TFFNP		# LC P  M   E:(-38+2NR) M:(-36+2NR)
					# SAVE ALSO FOR VGAMCALC
			TFF1/4
		DDV	PDVL		# (2/RMAG)  1/M   E:(26-NR)  M:(24-NR)
			NRMAG		# RMAG  M  E:(-29+NR) M:(-27+NR)
			VONE'		# SAVED VN.  E:(10)   M: (9)
		VSQ	DCOMP		# KEEP MPAC+2 HONEST FOR SQRT.
		STORE	TFFVSQ		# -(V SQ/MU)   E:(20)   M:(18)
					# SAVE FOR VGAMCALC
		SR*	DAD
# Page 1272
			0 -6,1		# GET -VSQ/MU   E:(26-NR)   M:(24-NR)
		STADR
					# 2/RMAG   FROM PDL+2
		STORE	TFFALFA		# ALFA  1/M   E:(26-NR)   M:(24-NR)
		SL*	PUSH		# TEMP SAVE ALFA    E:(20)   M:(18)
			0 -6,1
		ABS	SQRT		# E:(10)   M:(9)
		NORM
			X2		# X2 = -NA
		STORE	TFFRTALF	# SQRT( ABS(ALFA) )  E:(10+NA)   M:(9+NA)
		DSQ	SIGN		# NOT SO ACCURATE, BUT OK
					# ALFA FROM PDL+2   E:(20)   M:(18)
		BZE	BDDV		# SET 1/ALFA =0, TO SHOW SMALL ALFA
			+2
			TFF1/4
 +2		STORE	TFF1/ALF	# 1/ALFA   E:(-22-2NA)    M:(-20-2NA)
DUMPCNIC	RVQ

#                                                                 39 W
# Page 1273
# SUBROUTINE NAME:  TFFRP/RA                                              DATE: 01.17.67
# MOD NO:  0                                                              LOG SECTION:  TIME OF FREE FALL
# MOD NO:  1    MOD BY:  RR BAIRNSFATHER      DATE:  11 APR 67
# MOD NO:  2    MOD BY:  RR BAIRNSFATHER      DATE:  21 MAR 68            ACCEPT DIFFERENT EARTH/MOON SCALES
#                                                                         ALSO IMPROVE ACCURACY OF RAPO.
# FUNCTIONAL DESCRIPTION:         USED BY CALCTPER AND TFF DISPLAYS TO CALCULATE PERIGEE RADIUS AND ALSO
#          APOGEE RADIUS FOR A GENERAL CONIC.
#          PROGRAM GIVES PERIGEE RADIUS AS               APOGEE RADIUS IS GIVEN BY
#                 RP = P /(1+E)                                 RA = (1+E) / ALFA
#          WHERE   2
#                 E  = 1 - P ALFA
#          IF RA IS NEGATIVE OR SHOWS DIVIDE OVERFLOW, THEN RA = POSMAX BECAUSE
#                 1. APOGEE RADIUS IS NOT MEANINGFUL FOR HYPERBOLA
#                 2. APOGEE RADIUS IS NOT DEFINED FOR PARABOLA
#                 3. APOGEE RADIUS EXCEEDS THE SCALING FOR ELLIPSE.
#          THIS SUBROUTINE REQUIRED THE SIGNED RECIPROCAL SEMI MAJ AXIS, ALFA, AND SEMI-LATUS RECTUM AS DATA.
# CALLING SEQUENCE:  CALL
#                           TFFRP/RA
#          PUSHLOC = PDL+0, ARBITRARY IF LEQ 10D
#          C(MPAC) UNSPECIFIED
#
# SUBROUTINES CALLED:  NONE
# NORMAL EXIT MODE:    RVQ
#          IF ELLIPSE, WITHIN NORMAL SCALING, RAPO IS CORRECT.
#          OTHERWISE, RAPO = POSMAX.
# ALARMS:  NONE
# OUTPUT:  STORED IN PUSH LIST AREA. SCALE OF OUTPUT AGREES WITH DATA SUPPLIED TO  TFF/CONIC.
#          RPER	  E:(-29)  M:(-27)  M     PERIGEE RADIUS                  DESTROYED BY CALCTFF/CALCTPER, TFFTRIG.
#          RAPO	  E:(-29)  M:(-27)  M     APOGEE  RADIUS                  WILL BE DESTROYED BY CALCTFF/CALCTPER
#          PUSHLOC AT PDL+0
# ERASABLE INITIALIZATION REQUIRED:
#          TFFALFA E:(26-NR)      M   1/SEMI MAJ AXIS                     LEFT BY TFFCONIC
#                  M:(24-NR)
#          TFFNP  E: (-38+2NR)    M  LC P, SEMI LATUS RECTUM              LEFT BY TFFCONIC
#                 M: (-36+2NR)
#          X1                     -NR, NORM COUNT OF RMAG                 LEFT BY TFFCONIC
#          X2                     -NA, NORM COUNT OF ALFA                 LEFT BY TFFCONIC
#
# DEBRIS:  QPRET, PDL+0 ... PDL+1

# Page 1274
RAPO		=	16D		# APOGEE RADIUS  M  E:(-29)  M:(-27)
RPER		=	14D		# PERIGEE RADIUS  M  E:(-29)  M:(-27)

TFFRP/RA	DLOAD	DMP
			TFFALFA		# ALFA  1/M  E:(26-NR)   M:(24-NR)
			TFFNP		# LC P  M E:(-38+2NR) M:(-36+2NR)
		SR*	DCOMP		# ALFA P (-12+NR)
			0 -8D,1		# ALFA P  (-4)
		DAD	ABS		# (DCOMP GIVES VALID TP RESULT FOR SQRT)
					# (ABS PROTECTS SQRT IF E IS VERY NEAR 0)
			DP2(-4)
		SQRT	DAD		# E SQ = (1- P ALFA)   (-4)
			TFF1/4
		PUSH	BDDV		# (1+E)  (-2)  TO PDL+0
			TFFNP		# LCP  M E:(-38+2NR)   M:(-36+2NR)
		SR*	SR*		# (DOES SR THEN SL TO AVOID OVFL)
			0,1		# X1=-NR
			0 -7,1		# (EFFECTIVE SL)
		STODL	RPER		# PERIGEE RADIUS  M   E:(-29) M:(-27)
					# (1+E)  (-2)   FROM PDL+0
		DMP	BOVB
			TFF1/ALF	# E:(-22-2NA) M:(-20-2NA)
			TCDANZIG	# CLEAR OVFIND, IF ON.
		BZE	SL*
			MAXRA		# SET POSMAX IF ALFA=0
			0 -5,2		# -5+NA
		SL*	BOV
			0,2
			MAXRA		# SET POSMAX IF OVFL.
		BPL			# CONTINUE WITH VALID RAPO.
			+3
MAXRA		DLOAD			# RAPO CALC IS NOT VALID.  SET RAPO =
			NEARONE		# POSMAX AS A TAG.
 +3		STORE	RAPO		# APOGEE RADIUS  M   E:(-29)    M:(-27)
DUMPRPRA	RVQ

#                                                                 30 W
# Page 1275
# SUBROUTINE NAME:     CALCTPER / CALCTFF                                 DATE:  01.29.67
# MOD NO:  0                                                              LOG SECTION:  TIME OF FREE FALL
# MOD BY:  RR BAIRNSFATHER
# MOD NO:  1           MOD BY:  RR BAIRNSFATHER	DATE: 21 MAR 67
# MOD NO:  2           MOD BY:  RR BAIRNSFATHER	DATE: 14 APR 67
# MOD BY:  3           MOD BY:  RR BAIRNSFATHER	DATE: 8 JUL 67            NEAR EARTH MUE AND NEG TFF (GONEPAST)
# MOD BY:  4           MOD BY:  RR BAIRNSFATHER	DATE: 21 NOV 67           ADD VARIABLE MU.
# MOD BY:  5           MOD BY:  RR BAIRNSFATHER	DATE: 21 MAR 68	          ACCEPT DIFFERENT EARTH/MOON SCALES
# FUNCTIONAL DESCRIPTION:      PROGRAM CALCULATES THE FREE-FALL TIME OF FLIGHT FROM PRESENT POSITION  RN  AND
#          VELOCITY  VN  TO A RADIUS LENGTH SPECIFIED BY  RTERM  , SUPPLIED BY THE USER. THE POSITION VECTOR
#          RN  MAY BE ON EITHER SIDE OF THE CONIC, BUT  RTERM  IS CONSIDERED ON THE INBOUND SIDE.
#          THE EQUATIONS ARE
#
#                 Q2 = -SQRT(RTERM (2-RTERM ALFA) - LCP)    (INBOUND SIDE))        LEQ +- LCE/SQRT(ALFA)
#                      _  _
#                 Q1 = RN.VN / SQRT(MU)                                            LEQ +- LCE/SQRT(ALFA)
#
#                 Z = NUM / DEN                                                    LEQ +- 1/SQRT(ALFA)
#
#          WHERE, IF INBOUND
#                 NUM = RTERM -RN                                                  LEQ +- 2 LCE/ALFA
#                 DEN = Q2+Q1                                                      LEQ +- 2 LCE/SQRT(ALFA)
#
#          AND, IF OUTBOUND
#                 NUM = Q2-Q1                                                      LEQ +- 2 LCE/SQRT(ALFA)
#                 DEN = 2 - ALFA (RTERM + RN) .                                    LEQ +- 2 LCE
#
#          IF     ALFA ZZ < 1.0	                (FOR ALL CONICS EXCEPT ELLIPSES HAVING ABS(DEL ECC ANOM) G 90 DEG)
#
#          THEN	  X = ALFA Z Z
#          AND    TFF = (RTERM +RN -2 ZZ T(X) ) Z/SQRT(MU)
#
#                 EXCEPT IF ALFA PNZ, AND IF TFF NEG,
#                 THEN   TFF = 2 PI /(ALFA SQRT(ALFA)) + TFF
#
#          OR IF  ALFA ZZ  GEQ  1.0             (FOR ELLIPSES HAVING ABS(DEL ECC ANOM) GEQ 90 DEG)
#          THEN	  X = 1/ALFA Z Z
#          AND    TFF = (PI/SQRT(ALFA) -Q2 +Q1 +2(X T(X) -1) /ALFA Z) /ALFA SQRT(MU)
#
#          WHERE  T(X) IS A POLYNOMIAL APPROXIMATION TO THE SERIES
#                            2     3                        2
#                 1/3 -X/5 +X /7 -X /9 ...                (X   <  1.0)
#
# CALLING SEQUENC:   TIME TO RTERM                           TIME TO PERIGEE
#                    CALL                                    CALL
#                           CALCTFF                                 CALCTPER
#                    C(MPAC) = TERMNL RAD  M                 C(MPAC) = PERIGEE RAD  M
#          FOR EITHER,   E: (-29)    M: (-27)
#          FOR EITHER, PUSHLOC = PDL+0 , ARBITRARY IF LEQ 8D.
# Page 1276
#
# SUBROUTINES CALLED:  T(X),  VIA RTB
# NORMAL EXIT MODE:  RVQ
#          HOWEVER, PROGRAM EXITS WITH ONE OF THE FOLLOWING VALUES FOR TFF (-28) CS  IN MPAC. USER MUST STORE.
#                 A.  TFF= FLIGHT TIME. NORMAL CASE FOR POSITIVE FLIGHT TIME LESS THAN ONE ORBITAL PERIOD.
#                 B.  (THIS OPTION IS NO LONGER USED.)
#                 C.  TFF = POSMAX.  THIS INDICATES THAT THE CONIC FROM THE PRESENT POSITION WILL NOT RETURN TO
#                     THE SPECIFIED ALTITUDE.  ALSO INDICATES OUTBOUND PARABOLA OR HYPERBOLA.
# OUTPUT:  C(MPAC)  (-28)  CS     TIME OF FLIGHT, OR TIME TO PERIGEE
#          TFFX	   (0)            X,                                      LEFT FOR ENTRY DISPLAY TFF ROUTINES
#          NRTERM  E: (-29+NR)    M  RTERM, WEIGHTED BY NR                LEFT FOR ENTRY DISPLAY TFF ROUTINES
#                  M:(-27+NR)
#          TFFTEM  E:(-59+2NR)    LCP Z Z SGN(SDELF)                      LEFT FOR ENTRY DISPLAY TFF ROUTINES
#                  M:(-55+2NR)    LCP /ALFA SGN(SDELF)                    LEFT FOR ENTRY DISPLAY TFF ROUTINES
#          NOTE:  TFFTEM = PDL 36D AND WILL BE DESTROYED BY .:UNIT:.
#          RMAG1  E:(-29)  M:(-27)  PDL 12 NOT TOUCHED.
#          TFFQ1  E:(-16)  M:(-15)  PDL 14D
#          TFFDELQ E:(-16) M:(-15)  PDL 10D
#          PUSHLOC AT PDL+0
# ERASABLE INITIALIZATION REQUIRED:
#          RONE   E:(-29)  M:(-27)  M   STATE VECTOR                      LEFT BY USER
#          VONE'  E:(+10)  M:(+9)   VN/SQRT(NU)                           LEFT BY TFF/CONIC
#          RMAG1  E:(-29)  M:(-27)  PRESENT RADIUS,  M                    LEFT BY TFFCONIC
#          C(MPAC)E:(-29)  M:(-27)  RTERM, TERMINAL RADIUS LENGTH, M      LEFT BY USER
#
#          THE FOLLOWING ARE STORED IN THE PUSH LIST AREA.
#          TFF/RTMU  E:(17)  M:(14)  1/SQRT(MU)                           LEFT BY TFFCONIC.
#          NRMAG  E:(-29+NR)   M  RMAG, NORMALIZED                        LEFT BY TFFCONIC
#                 M:(-27+NR)
#          X1                     -NR, NORM COUNT                         LEFT BY TFFCONIC
#          TFFNP  E: (-38+2NR)  M   LCP, SEMI LATUS RECTUM, WEIGHT NR     LEFT BY TFFCONIC
#                 M: (-36+2N4)
#          TFFALFA  E: (26-NR)    1/M  ALFA, WEIGHT  NR                   LEFT BY TFFCONIC
#                   M: (24-NR)
#          TFFRTALF  E:(10+NA)    SQRT(ALFA), NORMALIZED                  LEFT BY TFFCONIC
#                    M:(9+NA)
#          X2                     -NA, NORMCOUNT                          LEFT BY TFFCONIC
#          TFF1/ALF  E: (-22-2NA)  SIGNED SEMIMAJ AXIS, WEIGHTED BY NA    LEFT BY TFFCONICf
#                    M: (-20-2NA)
# DEBRIS:  QPRET,   PDL+0 ... PDL+3
#          RTERM  E:(-29)  M(-27)   RTERM, TERMINAL RADIUS LENGTH
#          RAPO   E:(-29)  M(-27)   PDL 16D  (=NRTERM)
#          RPER   E:(-29)  M(-27)   PDL 14D   (=TFFQ1)

# Page 1277
CALCTPER	SETGO			# ENTER WITH RPER  IN MPAC
			TFFSW
			+3
CALCTFF		CLEAR			# ENTER WITH RTERM IN MPAC
			TFFSW
 +3		STORE	RTERM		# E: (-29)  M: (-27)
		SL*
			0,1		# X1=-NR
		STORE	NRTERM		# RTERM  E: (-29+NR) M: (-27+NR)
		DMP	BDSU
			TFFALFA		# ALFA  E: (26-NR) M: (24-NR)
			TFF1/4
		PUSH	DMP		# (2-ALFA RTERM)  (-3)  TO PDL+0
			NRTERM		# E: (-29+NR)  M: (-27+NR)
		PDDL	SR*		# RTERM(2-ALFA RTERM) TO PDL+2
					# E: (-32+NR)   M: (-30+NR)
			TFFNP		# LC P  E:(-38+2NR)  M:(-36+2NR)
			0 -6,1		# X1 = -NR
		DCOMP	DAD		# DUE TO SHIFTS, KEEP PRECISION FOR SQRT
					# RTERM(2-ALFA RTERM) FROM PDL+2
					# E: (-32+NR)  M: (-30+NR)
		SR*			# LEAVE  E: (-32)  M: (-30)
			0,1		# X1 = -NR
		BOFF	DLOAD		# CHECK TFF / TPER SWITCH
			TFFSW
			+2		# IF TFF, CONTINUE
			TFFZEROS	# IF TPER, SET Q2 = 0
 +2		BMN	SQRT		# E: (-16)  M: (-15)

			MAXTFF1		# NO FREE FALL CONIC TO RTERM FROM HERE
					# RESET PDL, SET TFF=POSMAX, AND EXIT.

		DCOMP	BOVB		# RT IS ON INBOUND SIDE.  ASSURE OVFIND=0
			TCDANZIG	# ANY PORT IN A STORM.
		STOVL	TFFTEM		# Q2   E: (-16)  M: (-15)
			VONE'		# VN/SQRT(MU) E: (10)  M: (9)
		DOT	SL3
			RONE		# SAVED RN.  E: (-29)  M: (-27)
		STORE	TFFQ1		# Q1, SAVE FOR GONEPAST TEST.
					# E: (-16)  M: (-15)
		BMN	BDSU
			INBOUND		# USE ALTERNATE Z
			TFFTEM		# Q2  E: (-16)  M: (-15)

# OUTBOUND Z CALC CONTINUES HERE

		STODL	TFFX		# NUM=Q2-Q1  E: (-16)  M: (-15)
			TFFALFA		# ALFA  E: (26-NR)  M: (24-NR)
		DMP	BDSU
# Page 1278
			NRMAG		# RMAG  E: (-29+NR)  M: (-27+NR)
					# (2-RTERM ALFA)  (-3) FROM PDL+0
SAVEDEN		PUSH	ABS		# DEN TO PDL+0  E: (-3) OR (-16)
					#               M: (-3) OR (-15)
		DAD	BOV		# INDETERMINANCY TEST
			LIM(-22)	# =1.0-B(-22)
			TFFXTEST	# GO IF DEN >/= B(-22)
		DLOAD	PDDL		# SET DEN=0  OTHERWISE
			TFFZEROS
					# XCH ZERO WITH PDL+0
		DLOAD	DCOMP
			TFFALFA		# ALFA  E: (26-NR)  M: (24-NR)
		BMN	DLOAD		# FOR TPER: Z INDET AT DELE/2=0 AND 90.
			TFFEL1		# ASSUME 90, AND LEAVE 0 IN PDL: 1/Z=D/N

					# Z INDET. AT PERIGEE FOR PARAB OR HYPERB.
DUMPTFF1	RVQ			# RETURN  TFF =0

# INBOUND Z CALC CONTINUES HERE

INBOUND		DLOAD			# RESET PDL+0
		DLOAD	DSU		# ALTERNATE Z CALC
			RTERM		# E: (-29)  M: (-27)
			RMAG1		# E: (-29)  M: (-27)
		STODL	TFFX		# NUM=RTERM-RN  E: (-29)  M: (-27)
			TFFTEM		# Q2  E: (-16)  M: (-15)
		DAD	GOTO
			TFFQ1		# Q1  E: (-16)  M: (-15)
			SAVEDEN		# DEN = Q2+Q1   E: (-16)  M: (-15)

TFFXTEST	DAD	PDDL		#  (ABS(DEN) TO PDL+2))  E: (-3) OR (-16)
					#                        M: (-3) OR (-15)
			DP(-22)		# RESTORE ABS(DEN) TO MPAC
			TFFX		# NUM  E:(-16) OR (-29)  M:(-15) OR (-27)
		DMP	SR*
			TFFRTALF	# SQRT(ALFA)  E:(10+NA) M:(9+NA)
			0 -3,2		# X2=-NA
		DDV			# C(MPAC) =NUM SQRT(ALFA)  E:(-3) OR (-16)
					#                          M:(-3) OR (-15)
					# ABS(DEN) FROM PDL+2  E:(-3) OR (-16)
					#                      M:(-3) OR (-15)
		DLOAD	BOV		# (THE DLOAD IS SHARED WITH TFFELL)
			TFFX		# NUM  E: (-16) OR (-29)  M:(-15) OR (-27)
			TFFELL		# USE EQN FOR DELE GEQ 90, LEQ -90

# OTHERWISE, CONTINUE FOR GENERAL CONIC FOR TFF EQN

		DDV	STADR
					# DEN FROM PDL+0   E: (-3) OR (-16)
					#                  M: (-3) OR (-15)
		STORE	TFFTEM		# Z SAVE FOR SIGN OF SDELF.
# Page 1279
					# E: (-13)  M: (-12)
		PUSH	DSQ		# Z TO PDL+0
		PUSH	DMP		# Z SQ TO PDL+2  E: (-26)  M: (-24)
			TFFNP		# LC P  E: (-38+2NR)  M: (-36+NR)
		SL	SIGN
			5
			TFFTEM		# AFFIX SIGN FOR SDELF (ENTRY DISPLAY)
		STODL	TFFTEM		# P ZSQ  E: (-59+2NR)  M: (-55+2NR)
					# (ARG IS USED IN TFF/TRIG)
					# ZSQ FROM PDL+2   E: (-26)  M: (-24)
		PUSH	DMP		# RESTORE PUSH LOC
			TFFALFA		# ALFA  E: (26-NR)  M: (24-NR)
		SL*
			0,1		# X1=-NR
		STORE	TFFX		# X
		RTB	DMP
			T(X)		#  POLY
					# ZSQ FROM PDL+2  E: (-26)  M: (-24)
		SR2	BDSU		# 2 ZSQ T(X)  E: (-29)  M: (-27)
			RTERM		# RTERM  E: (-29)  M: (-27)
		DAD	DMP
			RMAG1		# E: (-29)  M: (-27)
					# Z FROM PDL+0  E: (-13)  M: (-12)
		SR3	BPL		# TFF SQRT(MU)   E: (-45)  M: (-42)
			ENDTFF		# (NO PUSH UP)
		PUSH	SIGN		# TFF SQRT(MU)  TO PDL+0
			TFFQ1		# Q1 FOR GONEPAST TEST
		BPL	DLOAD		# GONE PAST ?
			NEGTFF		# YES. TFF < 0 .
			TFF1/ALF	# 1/ALFA  E: (-22-2NA)  M: (-20-2NA)
		DCOMP	BPL		# ALFA > 0 ?
			NEGTFF		# NO. TFF IS NEGATIVE.

# CORRECT FOR ORBITAL PERIOD.

		DCOMP			# YES. CORRECT FOR ORB PERIOD.
		DMP	DDV
			PI/16		# 2 PI (-5)
			TFFRTALF	# SQRT(ALFA)  E: (10+NA)  M: (9+NA)
		SL*	SL*
			0 -4,2		# X2=-NA
			0 -4,2
		SL*	DAD
			0,2
					# TFF SQRT(MU) FROM PDL+0  E:(-45) M:(-42)
ENDTFF		DMP	BOV		# TFF SQRT(MU) IN MPAC     E:(-45) M:(-42)
			TFF/RTMU	# E: (17)  M: (14)
			MAXTFF		# SET POSMAX IN OVFL.

DUMPTFF2	RVQ			# RETURN  TFF   (-28) CS IN MPAC.

# Page 1280
NEGTFF		DLOAD
					# TFF SQRT(MU)  FROM PDL+0, NEGATIVE.
		GOTO
			ENDTFF

MAXTFF1		DLOAD			# RESET PDL
MAXTFF		DLOAD	RVQ
			NEARONE

# TIME OF FLIGHT ELLIPSE WHEN DEL (ECCENTRIC ANOM) GEQ 90 AND LEQ -90.

					# NUM FROM TFFX.  E: (-16) OR (-29)
					#                 M: (-15) OR (-27)
TFFELL		SL2			# NUM  E:(-14) OR (-27)  M:(-13) OR (-25)
		BDDV	PUSH		# TEMP SAVE D/N IN PDL+0
					# DEN FROM PDL+0 E:(-3)/(-16) M:(-3)/(-15)
					# N/D TO PDL+0  E: (11)  M: (10)
TFFEL1		DLOAD	DSU		# (ENTER WITH D/N=0 IN PDL+0)
			TFFTEM		# Q2  E: (-16)  M: (-15)
			TFFQ1		# Q1  E: (-16)  M: (-15)
		STODL	TFFDELQ		# Q2-Q1  E: (-16)  M: (-15)
					# D/N FROM PDL+0
		STADR
		STORE	TFFTEM		# D/N  E: (11)  M: (10)
		DMP	SL*
			TFF1/ALF	# 1/ALFA E: (-22-2NA)  M: (-20-2NA)
			0,2		# 1/ALFA Z  E: (-11-NA)  M: (-10-NA)
		PUSH	DMP		# TO PDL+0
			TFFTEM		# 1/Z  E: (11)  M: (10)
		SL*	BOVB
			0,2		# X2= -NA
			SIGNMPAC	# IN CASE X= 1.0, CONTINUE
		STORE	TFFX		# X=1/ALFA ZSQ
		RTB	DMP
			T(X)		# POLY
			TFFX
		SR3	DSU
			DP2(-3)
		DMP	PUSH		# 2(X T(X)-1) /Z ALFA	E: (-15-NA)
					#			M: (-14-NA)
					# 1/ALFA Z FROM PDL+0	E: (-11-NA)
					#			M: (-10-NA)
		DLOAD	DMP		# GET SIGN FOR SDELF
			TFFTEM		# 1/Z  E: (11)  M: (10)
			RMAG1		# E: (-29)  M: (-27)
		SL2	DAD
			TFFQ1		# Q1  E: (-16)  M: (-15)
		STODL	TFFTEM		# (Q1+R 1/Z)  =SGN OF SDELF E:(-16) M:(-15)
			TFFNP		# LC P E: (-38+2NR) M: (-36+2NR)
		DMP	SL*		# CALC FOR ARG FOR TFF/TRIG.
# Page 1281
			TFF1/ALF	# 1/ALFA  E:(-22-2NA)  M:(-20-2NA)
			1,2		# X2=-NA
		SIGN	SL*
			TFFTEM		# AFFIX SIGN FOR SDELF
			0,2
		STODL	TFFTEM		# P/ALFA  E:(-59+2NR)  M:(-55+2NR)
					# (ARG FOR USE IN TFF/TRIG)
			TFF1/ALF	# 1/ALFA E:(-22-2NA)  M:(-20-2NA)
		SQRT	DMP
			PI/16		# PI (-4)
		DAD
					# 2(XT(X)-1)/Z ALFA FROM PDL E:(-15-NA)
					#                            M:(-14-NA)
		SL*	DSU
			0 -1,2
			TFFDELQ		# Q2-Q1  E: (-16)  M: (-15)
		DMP	SL*
			TFF1/ALF	# 1/ALFA  E:(-22-2NA)  M:(-20-2NA)
			0 -3,2
		SL*	GOTO
			0 -4,2
			ENDTFF		# TFF SQRT(MU) IN MPAC E:(-45) M:(-42)

# Page 1282
# PROGRAM NAME:      T(X)                                                 DATE:    01.17.67
# MOD NO:  0                                                              LOG SECTION:    TIME OF FREE FALL
# MOD BY:  RR BAIRNSFATHER
# FUNCTIONAL DESCRIPTION:      THE POLYNOMIAL T(X) IS USED BY TIME OF FLIGHT SUBROUTINES  CALCTFF AND
#          CALCTPER TO APPROXIMATE THE SERIES
#                            2     3
#                 1/3 -X/5 +X /7 -X /9 ...
#
#          WHERE  X = ALFA Z Z       IF ALFA Z Z LEQ 1
#                 X = 1/(ALFA Z Z)   IF ALFA Z Z  G  1
#
#          ALSO	  X IS NEG FOR HYPERBOLIC ORBITS
#                 X = 0 FOR PARABOLIC ORBITS
#                 X IS POSITIVE FOR ELLIPTIC ORBITS
#          FOR FLIGHT 278, THE POLYNOMIAL  T(X) IS FITTED OVER THE RANGE  (0,+1)  AND HAS A MAXIMUM
#          DEVIATION FROM THE SERIES OF  2 E-5       (T(X) IS A CHEBYCHEV  TYPE FIT AND WAS OBTAINED USING
#          MAX PROGRAM AUTCURFIT294RRB  AND IS VALID TO THE SAME TOLERANCE OVER THE RANGE  (-.08,+1). )
# CALLING SEQUENCE:  RTB
#                           T(X)
#          C(MPAC) = X
#
# SUBROUTINES CALLED:  NONE
# NORMAL EXIT MODE:    TC   DANZIG
# ALARMS:  NONE
# OUTPUT:  C(MPAC) = T(X)
# ERASABLE INITIALIZATION REQUIRED:
#          C(MPAC) = X
# DEBRIS:  NONE

T(X)		TC	POLY
		DEC	4		 # N-1
		2DEC	3.333333333 E-1

		2DEC*	-1.999819135 E-1 *

		2DEC*	1.418148467  E-1 *

		2DEC* 	-1.01310997  E-1 *

		2DEC*	5.609004986  E-2 *

		2DEC*	-1.536156925 E-2 *

ENDT(X)		TC	DANZIG

TCDANZIG	=	ENDT(X)

# Page 1283
# TFF CONSTANTS

		BANK	32

		SETLOC	TOF-FF1
		BANK

#                            NOTE _  NOTE _ ADJUSTED MUE FOR NEAR EARTH TRAJ.

#MUE		=	3.990 815 471 E10  M CUBE/CS SQ
#RTMUE		=	1.997702549 E5 B-18*      MODIFIED EARTH MU

#                            NOTE _  NOTE _ ADJUSTED MUE FOR NEAR EARTH TRAJ.

#MUM		=	4.902 778  E8     M CUBE /CS SQ

#RTMUM		2DEC*	2.21422176 E4 B-18*
PI/16		2DEC	3.141592653 B-4

LIM(-22)	2OCT	37777 37700		# 1.0 -B(-22)

DP(-22)		2OCT	00000 00100		# B(-22)

DP2(-3)		2DEC	1  B-3

DP2(-4)		2DEC	1 B-4			# 1/16

# RPAD1		2DEC	6373338 B-29    M  (-29)  =20 909 901.57 FT

RPAD1		=	RPAD
R300K		2DEC	6464778 B-29		#  (-29)  M

NEARONE		2DEC	.999999999

TFFZEROS	EQUALS	HI6ZEROS
TFF1/4		EQUALS	HIDP1/4

