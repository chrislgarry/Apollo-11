# Copyright:	Public domain.
# Filename:	INTERRUPT_LEAD_INS.agc
# Purpose:	Part of the source code for Comanche, build 055.
#		It is part of the source code for the Command Module's (CM)
#		Apollo Guidance Computer (AGC), Apollo 11.
# Assembler:	yaYUL
# Reference:	pp. 131-132
# Contact:	Ron Burkey <info@sandroid.org>,
#  		Fabrizio Bernardini <fabrizio@spacecraft.it>
# Website:	http://www.ibiblio.org/apollo.
# Mod history:	09/05/09 FB	Transcription of Batch FB-1 Assignment.
#
# The contents of the "Comanche055" files, in general, are transcribed 
# from scanned documents. 
#
#	Assemble revision 055 of AGC program Comanche by NASA
#	2021113-051.  April 1, 1969.  
#
#	This AGC program shall also be referred to as Colossus 2A
#
#	Prepared by
#			Massachussets Institute of Technology
#			75 Cambridge Parkway
#			Cambridge, Massachusetts
#
#	under NASA contract NAS 9-4065.
#
# Refer directly to the online document mentioned above for further
# information.  Please report any errors to info@sandroid.org.

# Page 131
		SETLOC	4000 
		
		COUNT	02/RUPTS
		
		INHINT			# GO
		CAF	GOBB
		XCH	BBANK
		TCF	GOPROG
		
		DXCH	ARUPT		# T6RUPT
		EXTEND
		DCA	T6LOC
		DTCB
		
		DXCH	ARUPT		# T5RUPT
		CS	TIME5
		AD	.5SEC
		TCF	T5RUPT
		
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
# Page 132
		CAF	RDRPTBB
		XCH	BBANK
		TCF	VHFREAD
		
		DXCH	ARUPT		# HAND CONTROL RUPT
		CAF	HCRUPTBB
		XCH	BBANK
		TCF	RESUME +3	# NOT USED
		
		EBANK=	LST1		# RESTART USES E0,E3
GOBB		BBCON	GOPROG

		EBANK=	LST1
T3RPTBB		BBCON	T3RUPT

		EBANK=	KEYTEMP1
KEYRPTBB	BBCON	KEYRUPT1

		EBANK=	MRKBUF1
MKRUPTBB	BBCON	MARKRUPT

UPRPTBB		=	KEYRPTBB

		EBANK=	DNTMBUFF
DWNRPTBB	BBCON	DODOWNTM

		EBANK=	DATATEST
RDRPTBB		BBCON	VHFREAD

		EBANK=	TIME1
HCRUPTBB	BBCON	RESUME		# NOT USED

		EBANK=	DSRUPTSW
T4RPTBB		BBCON	T4RUPT

		EBANK=	TIME1
T5RPTBB		BBCON	T5RUPT

T5RUPT		EXTEND
		BZMF	NOQBRSM
		EXTEND
		DCA	T5LOC
		DTCB
		
		

