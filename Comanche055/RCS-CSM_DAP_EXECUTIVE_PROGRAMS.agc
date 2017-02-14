# Copyright:	Public domain.
# Filename:	RCS-CSM_DAP_EXECUTIVE_PROGRAMS.agc
# Purpose:	Part of the source code for Colossus 2A, AKA Comanche 055.
#		It is part of the source code for the Command Module's (CM)
#		Apollo Guidance Computer (AGC), for Apollo 11.
# Assembler:	yaYUL
# Contact:	Ron Burkey <info@sandroid.org>.
# Website:	www.ibiblio.org/apollo.
# Pages:	1037-1038
# Mod history:	2009-05-13 RSB	Adapted from the Colossus249/ file of the
#				same name, using Comanche055 page images.
#		2009-05-20 RSB	A "Page N" comment was corrected.
#
# This source code has been transcribed or otherwise adapted from digitized
# images of a hardcopy from the MIT Museum.  The digitization was performed
# by Paul Fjeld, and arranged for by Deborah Douglas of the Museum.  Many
# thanks to both.  The images (with suitable reduction in storage size and
# consequent reduction in image quality as well) are available online at
# www.ibiblio.org/apollo.  If for some reason you find that the images are
# illegible, contact me at info@sandroid.org about getting access to the
# (much) higher-quality images which Paul actually created.
#
# Notations on the hardcopy document read, in part:
#
#	Assemble revision 055 of AGC program Comanche by NASA
#	2021113-051.  10:28 APR. 1, 1969
#
#	This AGC program shall also be referred to as
#			Colossus 2A

# Page 1037
# CALCULATION OF  AMGB, AMBG  ONCE EVERY SECOND
#
#	AMGB =	1	SIN(PSI)		0
#		0	COS(PSI)COS(PHI)	SIN(PHI)
#		0	-COS(PSI)SIN(PHI)	COS(PHI)
#
#	AMBG =	1	-TAN(PSI)COS(PHI)	TAN(PSI)SIN(PHI)
#		0	COS(PHI)/COS(PSI)	-SIN(PHI)/COS(PSI)
#		0	SIN(PHI)		COS(PHI)
#
# WHERE PHI AND PSI ARE CDU ANGLES


		BANK	20
		SETLOC	DAPS8
		BANK

		COUNT*	$$/DAPEX
		EBANK=	KMPAC
AMBGUPDT	CA	FLAGWRD6	# CHECK FOR RCS AUTOPILOT
		EXTEND
		BZMF	ENDOFJOB	# BIT15 = 0, BIT14 = 1
		MASK	BIT14		# IF NOT RCS, EXIT
		EXTEND
		BZF	ENDOFJOB	# TO PROTECT TVC DAP ON SWITCHOVER

		CA	CDUZ
		TC	SPSIN2
		TS	AMGB1		# CALCULATE AMGB
		CA	CDUZ
		TC	SPCOS2
		TS	CAPSI		# MUST CHECK FOR GIMBAL LOCK
		CAF	QUADANGL	# = 7.25  DEGREES JET QUAD ANGULAR OFFSET
		EXTEND
		MSU	CDUX
		COM			# CDUX - 7.25 DEG
		TC	SPCOS1
		TS	AMGB8
		EXTEND
		MP	CAPSI
		TS	AMGB4
		CAF	QUADANGL
		EXTEND
		MSU	CDUX
		COM			# CDUX - 7.25 DEG
		TC	SPSIN1
		TS	AMGB5
		EXTEND
		MP	CAPSI
		COM
# Page 1038
		TS	AMGB7
		TCF	ENDOFJOB
QUADANGL	DEC	660		# = 7.25 DEGREES
