# Copyright:	Public domain.
# Filename:	MAIN.agc
# Purpose:	Part of the source code for Colossus 2A, AKA Comanche 055.
#		It is part of the source code for the Command Module's (CM)
#		Apollo Guidance Computer (AGC) Apollo 11.
# Assembler:	yaYUL
# Contact:	Ron Burkey <info@sandroid.org>.
# Website:	www.ibiblio.org/apollo
# Mod history:	2009-05-05 RSB	Adapted from Colossus249/MAIN.agc.
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
#
# This file is a little different from the other Comanche055 files I'm providing, 
# in that it doesn't represent anything that appears directly in the original source.  
# What I (RSB) have done for organizational purposes is to split the huge monolithic
# source code into smaller, more manageable chunks--i.e., into individual source 
# files.  Those files are rejoined within this file as "includes".  It just makes
# it a little easier to work with.  The code chunks correspond to natural divisions
# into sub-programs.  In fact, these divisions are more-or-less specified by
# the source code itself.  Refer to the "SUBROUTINE CALLS" at the
# very beginning of the file ASSEMBLY_AND_OPERATION_INFORMATION.agc.
#
# It may be reasonably asked why tens of thousands of lines of source are joined by
# means of inclusion, rather than simply assembling the source files individually and
# then linking them to form the executable.  The answer is that the original 
# development team had no linker.  The builds were monolithic just like this.
# There was a big emphasis on reusability of the code in the original project, 
# apparently, but this reusability took the form of inserting your deck of 
# punch-cards at the appropriate position in somebody else's deck of punch-cards.
# (Actually, I think the card-decks were turned into tape libraries, and the modules
# were mixed-and-matched from the tape libraries, but the principle is the same.)
# So, indeed, the method of file-inclusion is a very fair representation of the 
# methods used in the original development ... with the improvement, of course,
# that you no longer have to worry about dropping the card deck.  On the other hand, 
# I wasn't there at the time, so I may have no idea what I'm talking about.
#
# Finally, note that the original Apollo AGC assembler (called "YUL") is no longer 
# available (as far as I can tell).  Actually, it had already been replaced by another
# assembler (called "GAP") by the time of Apollo 11, but GAP isn't available either.
# The replacement assembler yaYUL accepts a slightly different format for the source 
# code from what YUL or GAP accepted, so the source code has been targeted for 
# assembly with yaYUL.

# What follows is simply a bunch of file-includes for the individual code chunks.
# I've marked the page numbers to make proof-reading easier.  The page images also
# contain a lot of interesting tables (cross-referenced to page numbers) created by GAP, 
# but not duplicated by yaYUL, so it's still valuable even if the source-files
# listed below are in hand.

$CONTRACT_AND_APPROVALS.agc			# p. 1						
$ASSEMBLY_AND_OPERATION_INFORMATION.agc		# pp. 2-26
$TAGS_FOR_RELATIVE_SETLOC.agc			# pp. 27-35

						# p. 36 contains no code.
# COMERASE
$ERASABLE_ASSIGNMENTS.agc			# pp. 37-130

# COMAID
$INTERRUPT_LEAD_INS.agc				# pp. 131-132
$T4RUPT_PROGRAM.agc				# pp. 133-169
$DOWNLINK_LISTS.agc				# pp. 170-180
$FRESH_START_AND_RESTART.agc			# pp. 181-210
$RESTART_TABLES.agc				# pp. 211-221
$SXTMARK.agc 					# pp. 222-235
$EXTENDED_VERBS.agc				# pp. 236-267
$PINBALL_NOUN_TABLES.agc			# pp. 268-284
$CSM_GEOMETRY.agc				# pp. 285-296
$IMU_COMPENSATION_PACKAGE.agc			# pp. 297-306
$PINBALL_GAME_BUTTONS_AND_LIGHTS.agc		# pp. 307-389
$R60_62.agc					# pp. 390-398
$ANGLFIND.agc					# pp. 399-411
$GIMBAL_LOCK_AVOIDANCE.agc			# pp. 412-413
$KALCMANU_STEERING.agc				# pp. 414-419
$SYSTEM_TEST_STANDARD_LEAD_INS.agc		# pp. 420-422
$IMU_CALIBRATION_AND_ALIGNMENT.agc		# pp. 423-455

# COMEKISS
$GROUND_TRACKING_DETERMINATION_PROGRAM.agc	# pp. 456-459
$P34-35_P74-75.agc				# pp. 460-504
$R31.agc					# pp. 505-510
$P76.agc					# pp. 511-513
$R30.agc					# pp. 514-524
$STABLE_ORBIT.agc				# pp. 525-532

# TROUBLE
$P11.agc					# pp. 533-550
$TPI_SEARCH.agc					# pp. 551-561
$P20-P25.agc					# pp. 562-634
$P30-P37.agc					# pp. 635-648
$P32-P33_P72-P73.agc				# pp. 649-683
$P40-P47.agc					# pp. 684-736
$P51-P53.agc					# pp. 737-784
$LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc	# pp. 785-788
$P61-P67.agc					# pp. 789-818
$SERVICER207.agc				# pp. 819-836
$ENTRY_LEXICON.agc				# pp. 837-843
$REENTRY_CONTROL.agc				# pp. 844-882
$CM_BODY_ATTITUDE.agc				# pp. 883-889
$P37_P70.agc					# pp. 890-933
$S-BAND_ANTENNA_FOR_CM.agc			# pp. 934-935
$LUNAR_LANDMARK_SELECTION_FOR_CM.agc		# pp. 936

# TVCDAPS
$TVCINITIALIZE.agc				# pp. 937-944
$TVCEXECUTIVE.agc				# pp. 945-950
$TVCMASSPROP.agc				# pp. 951-955
$TVCRESTARTS.agc				# pp. 956-960
$TVCDAPS.agc					# pp. 961-978
$TVCSTROKETEST.agc				# pp. 979-983
$TVCROLLDAP.agc					# pp. 984-998
$MYSUBS.agc					# pp. 999-1001
$RCS-CSM_DIGITAL_AUTOPILOT.agc			# pp. 1002-1024
$AUTOMATIC_MANEUVERS.agc			# pp. 1025-1036
$RCS-CSM_DAP_EXECUTIVE_PROGRAMS.agc		# pp. 1037-1038
$JET_SELECTION_LOGIC.agc			# pp. 1039-1062
$CM_ENTRY_DIGITAL_AUTOPILOT.agc			# pp. 1063-1092

# CHIEFTAN
$DOWN-TELEMETRY_PROGRAM.agc			# pp. 1093-1102
$INTER-BANK_COMMUNICATION.agc			# pp. 1103-1106
$INTERPRETER.agc				# pp. 1107-1199
$FIXED_FIXED_CONSTANT_POOL.agc			# pp. 1200-1204
$INTERPRETIVE_CONSTANTS.agc			# pp. 1205-1206
$SINGLE_PRECISION_SUBROUTINES.agc		# p.  1207
$EXECUTIVE.agc					# pp. 1208-1220
$WAITLIST.agc					# pp. 1221-1235
$LATITUDE_LONGITUDE_SUBROUTINES.agc		# pp. 1236-1242
$PLANETARY_INERTIAL_ORIENTATION.agc		# pp. 1243-1251
$MEASUREMENT_INCORPORATION.agc			# pp. 1252-1261
$CONIC_SUBROUTINES.agc				# pp. 1262-1308
$INTEGRATION_INITIALIZATION.agc			# pp. 1309-1333
$ORBITAL_INTEGRATION.agc			# pp. 1334-1354
$INFLIGHT_ALIGNMENT_ROUTINES.agc		# pp. 1355-1364
$POWERED_FLIGHT_SUBROUTINES.agc			# pp. 1365-1372
$TIME_OF_FREE_FALL.agc				# pp. 1373-1388
$STAR_TABLES.agc				# pp. 1389-1393
$AGC_BLOCK_TWO_SELF-CHECK.agc			# pp. 1394-1403
$PHASE_TABLE_MAINTENANCE.agc			# pp. 1404-1413
$RESTARTS_ROUTINE.agc				# pp. 1414-1419
$IMU_MODE_SWITCHING_ROUTINES.agc		# pp. 1420-1448
$KEYRUPT_UPRUPT.agc				# pp. 1449-1451
$DISPLAY_INTERFACE_ROUTINES.agc			# pp. 1452-1484
$SERVICE_ROUTINES.agc				# pp. 1485-1492
$ALARM_AND_ABORT.agc				# pp. 1493-1496
$UPDATE_PROGRAM.agc				# pp. 1497-1507
$RT8_OP_CODES.agc				# pp. 1508-1516

						# pp. 1517-1751: GAP-generated tables.





