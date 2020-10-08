# Copyright:	Public domain.
# Filename:	INTERRUT_LEAD_INS.agc
# Purpose: 	Part of the source code for Luminary 1A build 099.
#		It is part of the source code for the Lunar Module's (LM)
#		Apollo Guidance Computer (AGC), for Apollo 11.
# Assembler:	yaYUL
# Contact:	Onno Hommes <ohommes@cmu.edu>.
# Website:	www.ibiblio.org/apollo.
# Pages:	0153-0154
# Mod history:	2009-05-14 OH	Transcribed from page images.
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


# Page 153
		SETLOC	4000

		COUNT*	$$/RUPTS	# FIX-FIX LEAD INS
		INHINT			# GO
		CAF	GOBB
		XCH	BBANK
		TCF	GOPROG

		DXCH	ARUPT		# T6RUPT
		EXTEND
		DCA	T6ADR
		DTCB

		DXCH	ARUPT		# T5RUPT - AUTOPILOT
		EXTEND
		DCA	T5ADR
		DTCB

		DXCH	ARUPT		# T3RUPT
		CAF	T3RPTBB
		XCH	BBANK
		TCF	T3RUPT

		DXCH	ARUPT		# T4RUPT
		CAF	T4RPTBB
		XCH	BBANK
		TCF	T4RUPT

		DXCH	ARUPT		# KEYRUPT1
		CAF	KEYRPTBB
		XCH	BBANK
		TCF	KEYRUPT1

		DXCH	ARUPT		# KEYRUPT2
		CAF	MKRUPTBB
		XCH	BBANK
		TCF	MARKRUPT

		DXCH	ARUPT		# UPRUPT
		CAF	UPRPTBB
		XCH	BBANK
		TCF	UPRUPT

		DXCH	ARUPT		# DOWNRUPT
		CAF	DWNRPTBB
		XCH	BBANK
		TCF	DODOWNTM

		DXCH	ARUPT		# RADAR RUPT
		CAF	RDRPTBB
# Page 154
		XCH	BBANK
		TCF	RADAREAD

		DXCH	ARUPT		# RUPT10 IS USED ONLY BY LANDING GUIDANCE
		CA	RUPT10BB
		XCH	BBANK
		TCF	PITFALL


		EBANK=	LST1		# RESTART USES E0, E3
GOBB		BBCON	GOPROG

		EBANK=	PERROR
T6ADR		2CADR	DOT6RUPT

		EBANK=	LST1
T3RPTBB		BBCON	T3RUPT

		EBANK=	KEYTEMP1
KEYRPTBB	BBCON	KEYRUPT1

		EBANK=	AOTAZ
MKRUPTBB	BBCON	MARKRUPT

UPRPTBB		=	KEYRPTBB

		EBANK=	DNTMBUFF
DWNRPTBB	BBCON	DODOWNTM

		EBANK=	RADMODES
RDRPTBB		BBCON	RADAREAD

		EBANK=	M11
T4RPTBB		BBCON	T4RUPT

		EBANK=	ELVIRA
RUPT10BB	BBCON	PITFALL

