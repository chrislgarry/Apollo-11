# Copyright:	Public domain.
# Filename:	KALMAN_FILTER.agc
# Purpose: 	Part of the source code for Luminary 1A build 099.
#		It is part of the source code for the Lunar Module's (LM)
#		Apollo Guidance Computer (AGC), for Apollo 11.
# Assembler:	yaYUL
# Contact:	Ron Burkey <info@sandroid.org>.
# Website:	www.ibiblio.org/apollo.
# Pages:	1470-1471
# Mod history:	2009-05-27 RSB	Adapted from the corresponding 
#				Luminary131 file, using page 
#				images from Luminary 1A.
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
#	Assemble revision 001 of AGC program LMY99 by NASA 2021112-61
#	16:27 JULY 14, 1969 

# Page 1470
		EBANK=	NO.UJETS
		BANK	16
		SETLOC	DAPS1
		BANK

		COUNT*	$$/DAP

RATELOOP	CA	TWO
		TS	DAPTEMP6
		DOUBLE
		TS	Q
		INDEX	DAPTEMP6
		CCS	TJP
		TCF	+2
		TCF	LOOPRATE
		AD	-100MST6
		EXTEND
		BZMF	SMALLTJU
		INDEX	DAPTEMP6
		CCS	TJP
		CA	-100MST6
		TCF	+2
		CS	-100MST6
		INDEX	DAPTEMP6
		ADS	TJP
		INDEX	DAPTEMP6
		CCS	TJP
		CS	-100MS		# 0.1 AT 1
		TCF	+2
		CA	-100MS
LOOPRATE	EXTEND
		INDEX	DAPTEMP6
		MP	NO.PJETS
		CA	L
		INDEX	DAPTEMP6
		TS	DAPTEMP1	# SIGNED TORQUE AT 1 JET-SEC FOR FILTER
		EXTEND
		MP	BIT10		# RESCALE TO 32; ONE BIT ABOUT 2 JET-MSEC
		EXTEND
		BZMF	NEGTORK
STORTORK	INDEX	Q		# INCREMENT DOWNLIST REGISTER.
		ADS	DOWNTORK	#	NOTE:  NOT INITIALIZED; OVERFLOWS.

		CCS	DAPTEMP6
		TCF	RATELOOP +1
		TCF	ROTORQUE
SMALLTJU	CA	ZERO
		INDEX	DAPTEMP6
		XCH	TJP
		EXTEND
# Page 1471
		MP	ELEVEN		# 10.24 PLUS
		CA	L
		TCF	LOOPRATE
ROTORQUE	CA	DAPTEMP2
		AD	DAPTEMP3
		EXTEND
		MP	1JACCR
		TS	JETRATER
		CS	DAPTEMP3
		AD	DAPTEMP2
		EXTEND
		MP	1JACCQ
		TS	JETRATEQ
		TCF	BACKP
-100MST6	DEC	-160

NEGTORK		COM
		INCR	Q
		TCF	STORTORK
		

