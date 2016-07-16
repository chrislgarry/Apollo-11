# Copyright:	Public domain.
# Filename:	GIMBAL_LOCK_AVOIDANCE.agc
# Purpose:	Part of the source code for Comanche, build 055.
#		It is part of the source code for the Command Module's (CM)
#		Apollo Guidance Computer (AGC), Apollo 11.
# Assembler:	yaYUL
# Reference:	pp. 412-413
# Contact:	Onno Hommes <ohommes@cmu.edu>.
# Website:	www.ibiblio.org/apollo.
# Mod history:	05/07/09 OH	Transcription Batch 1 Assignment 
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
#			Massachusetts Institute of Technology
#			75 Cambridge Parkway
#			Cambridge, Massachusetts
#
#	under NASA contract NAS 9-4065.
#
# Refer directly to the online document mentioned above for further information.
# Please report any errors to info@sandroid.org.


# Page 412
		BANK	15		
		SETLOC	KALCMON1
		BANK
		
		EBANK=	BCDU
		
# DETECTING GIMBAL LOCK
LOCSKIRT	EQUALS	WCALC
WCALC		LXC,1	DLOAD*
			RATEINDX
			ARATE,1	
		SR4	CALL		# COMPUTE THE INCREMENTAL ROTATION MATRIX
			DELCOMP		# DEL CORRESPONDING TO A 1 SEC ROTATION
					# ABOUT COF
		DLOAD*	VXSC
			ARATE,1
			COF
		MXV
			QUADROT
		STODL	BRATE	
			AM
		DMP	DDV*
			ANGLTIME
			ARATE,1
		SR
			5
		STOVL	TM
			BRATE
		VXSC
			BIASCALE
		STORE	BIASTEMP	# ATTITUDE ERROR BIAS TO PREVENT OVERSHOOT
					# IN SYSTEM
		SETGO			# STATE SWITCH CALCMAN2 (43D)
			CALCMAN2	# 0(OFF) = BYPASS STARTING PROCEDURE
			NEWANGL +1	# 1(ON) = START MANEUVER
			
			
ARATE		2DEC	.0022222222	# = .05 DEG/SEC

		2DEC	.0088888889	# = .2 DEG/SEC
		
		2DEC	.0222222222	# = .5 DEG/SEC
		
		2DEC	.0888888889	# = 2 DEG/SEC                $22.5 DEG/SEC
		
ANGLTIME	2DEC	.000190735	# = 100B - 19

					# MANEUVER ANGLE TO MANEUVER TIME
QUADROT		2DEC	.1		# ROTATION MATRIX FROM S/C AXES TO CONTROL

# Page 413
		2DEC	0		# AXES (X ROT = -7.25 DEG)
		
		2DEC	0
		
		2DEC	0
		
		2DEC	.099200		# =(.1)COS7.25
		
		2DEC	-.012620	# =-(.1)SIN7.25
		
		2DEC	0
		
		2DEC	.012620		# (.1)SIN7.25
		
		2DEC	.099200		# (.1)COS7.25
		
BIASCALE	2DEC	.0002543132	# = (450/180)(1/0.6)(1/16384)
