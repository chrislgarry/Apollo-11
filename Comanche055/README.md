# Comanche055

The images (with suitable reduction in storage size and consequent reduction in image quality as well) are available online at www.ibiblio.org/apollo. If for some reason you find that the images are illegible, contact me at info@sandroid.org about getting access to the (much) higher-quality images which Paul actually created.

## Background

For organizatinal purposes RSB split the huge monolithic source code into smaller, more manageable chunks--i.e., into individual source
files. Those files are rejoined as "includes". The code chunks correspond to natural divisions into sub-programs. In fact, these divisions are more-or-less specified by the source code itself. Refer to the `"SUBROUTINE CALLS"` at the very beginning of `ASSEMBLY_AND_OPERATION_INFORMATION.agc`.

It may be reasonably asked why tens of thousands of lines of source are joined by means of inclusion, rather than simply assembling the source files individually and then linking them to form the executable. The answer is that the original development team had no linker. The builds were monolithic just like this.

There was a big emphasis on reusability of the code in the original project, apparently, but this reusability took the form of inserting your deck of punch-cards at the appropriate position in somebody else's deck of punch-cards. (Actually, I think the card-decks were turned into tape libraries, and the modules were mixed-and-matched from the tape libraries, but the principle is the same.) So, indeed, the method of file-inclusion is a very fair representation of the methods used in the original development...with the improvement, of course,
that you no longer have to worry about dropping the card deck. On the other hand, I (RSB) wasn't there at the time, so I may have no idea what I'm talking about.

Finally, note that the original Apollo AGC assembler (called `YUL`) is no longer available (as far as I can tell). Actually, it had already been replaced by another assembler (called `GAP`) by the time of Apollo 11, but GAP isn't available either. The replacement assembler yaYUL accepts a slightly different format for the source code from what YUL or GAP accepted, so the source code has been targeted for assembly with yaYUL.

What follows is simply a bunch of file-includes for the individual code chunks. The page numbers have been marked to make proof-reading easier. The page images also contain a lot of interesting tables (cross-referenced to page numbers) created by GAP, but not duplicated by yaYUL, so it's still valuable even if the source-files listed below are in hand.

## Source Code Index

### INFORMATION

Source File                              | Page Number
:--------------------------------------- | :----------
[CONTRACT_AND_APPROVALS.agc]             | 1
[ASSEMBLY_AND_OPERATION_INFORMATION.agc] | 2-26
[TAGS_FOR_RELATIVE_SETLOC.agc]           | 27-35

### COMERASE

Source File                | Page Number
:------------------------- | :----------
[ERASABLE_ASSIGNMENTS.agc] | 37-130

### COMAID

Source File                           | Page Number
:------------------------------------ | :----------
[INTERRUPT_LEAD_INS.agc]              | 131-132
[T4RUPT_PROGRAM.agc]                  | 133-169
[DOWNLINK_LISTS.agc]                  | 170-180
[FRESH_START_AND_RESTART.agc]         | 181-210
[RESTART_TABLES.agc]                  | 211-221
[SXTMARK.agc]                         | 222-235
[EXTENDED_VERBS.agc]                  | 236-267
[PINBALL_NOUN_TABLES.agc]             | 268-284
[CSM_GEOMETRY.agc]                    | 285-296
[IMU_COMPENSATION_PACKAGE.agc]        | 297-306
[PINBALL_GAME_BUTTONS_AND_LIGHTS.agc] | 307-389
[R60_62.agc]                          | 390-398
[ANGLFIND.agc]                        | 399-411
[GIMBAL_LOCK_AVOIDANCE.agc]           | 412-413
[KALCMANU_STEERING.agc]               | 414-419
[SYSTEM_TEST_STANDARD_LEAD_INS.agc]   | 420-422
[IMU_CALIBRATION_AND_ALIGNMENT.agc]   | 423-455

### COMEKISS

Source File                                 | Page Number
:------------------------------------------ | :----------
[GROUND_TRACKING_DETERMINATION_PROGRAM.agc] | 456-459
[P34-35_P74-75.agc]                         | 460-504
[R31.agc]                                   | 505-510
[P76.agc]                                   | 511-513
[R30.agc]                                   | 514-524
[STABLE_ORBIT.agc]                          | 525-532

### TROUBLE

Source File                                   | Page Number
:-------------------------------------------- | :----------
[P11.agc]                                     | 533-550
[TPI_SEARCH.agc]                              | 551-561
[P20-P25.agc]                                 | 562-634
[P30-P37.agc]                                 | 635-648
[P32-P33_P72-P73.agc]                         | 649-683
[P40-P47.agc]                                 | 684-736
[P51-P53.agc]                                 | 737-784
[LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc] | 785-788
[P61-P67.agc]                                 | 789-818
[SERVICER207.agc]                             | 819-836
[ENTRY_LEXICON.agc]                           | 837-843
[REENTRY_CONTROL.agc]                         | 844-882
[CM_BODY_ATTITUDE.agc]                        | 883-889
[P37_P70.agc]                                 | 890-933
[S-BAND_ANTENNA_FOR_CM.agc]                   | 934-935
[LUNAR_LANDMARK_SELECTION_FOR_CM.agc]         | 936

### TVCDAPS

Source File                          | Page Number
:----------------------------------- | :----------
[TVCINITIALIZE.agc]                  | 937-944
[TVCEXECUTIVE.agc]                   | 945-950
[TVCMASSPROP.agc]                    | 951-955
[TVCRESTARTS.agc]                    | 956-960
[TVCDAPS.agc]                        | 961-978
[TVCSTROKETEST.agc]                  | 979-983
[TVCROLLDAP.agc]                     | 984-998
[MYSUBS.agc]                         | 999-1001
[RCS-CSM_DIGITAL_AUTOPILOT.agc]      | 1002-1024
[AUTOMATIC_MANEUVERS.agc]            | 1025-1036
[RCS-CSM_DAP_EXECUTIVE_PROGRAMS.agc] | 1037-1038
[JET_SELECTION_LOGIC.agc]            | 1039-1062
[CM_ENTRY_DIGITAL_AUTOPILOT.agc]     | 1063-1092

### CHIEFTAN

Source File                          | Page Number
:----------------------------------- | :----------
[DOWN-TELEMETRY_PROGRAM.agc]         | 1093-1102
[INTER-BANK_COMMUNICATION.agc]       | 1103-1106
[INTERPRETER.agc]                    | 1107-1199
[FIXED_FIXED_CONSTANT_POOL.agc]      | 1200-1204
[INTERPRETIVE_CONSTANTS.agc]         | 1205-1206
[SINGLE_PRECISION_SUBROUTINES.agc]   | 1207
[EXECUTIVE.agc]                      | 1208-1220
[WAITLIST.agc]                       | 1221-1235
[LATITUDE_LONGITUDE_SUBROUTINES.agc] | 1236-1242
[PLANETARY_INERTIAL_ORIENTATION.agc] | 1243-1251
[MEASUREMENT_INCORPORATION.agc]      | 1252-1261
[CONIC_SUBROUTINES.agc]              | 1262-1308
[INTEGRATION_INITIALIZATION.agc]     | 1309-1333
[ORBITAL_INTEGRATION.agc]            | 1334-1354
[INFLIGHT_ALIGNMENT_ROUTINES.agc]    | 1355-1364
[POWERED_FLIGHT_SUBROUTINES.agc]     | 1365-1372
[TIME_OF_FREE_FALL.agc]              | 1373-1388
[STAR_TABLES.agc]                    | 1389-1393
[AGC_BLOCK_TWO_SELF-CHECK.agc]       | 1394-1403
[PHASE_TABLE_MAINTENANCE.agc]        | 1404-1413
[RESTARTS_ROUTINE.agc]               | 1414-1419
[IMU_MODE_SWITCHING_ROUTINES.agc]    | 1420-1448
[KEYRUPT_UPRUPT.agc]                 | 1449-1451
[DISPLAY_INTERFACE_ROUTINES.agc]     | 1452-1484
[SERVICE_ROUTINES.agc]               | 1485-1492
[ALARM_AND_ABORT.agc]                | 1493-1496
[UPDATE_PROGRAM.agc]                 | 1497-1507
[RT8_OP_CODES.agc]                   | 1508-1516

### MISCELLANEOUS

Source File          | Page Number
:------------------- | :----------
GAP-generated tables | 1517-1751

[CONTRACT_AND_APPROVALS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONTRACT_AND_APPROVALS.agc
[ASSEMBLY_AND_OPERATION_INFORMATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ASSEMBLY_AND_OPERATION_INFORMATION.agc
[TAGS_FOR_RELATIVE_SETLOC.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TAGS_FOR_RELATIVE_SETLOC.agc
[ERASABLE_ASSIGNMENTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ERASABLE_ASSIGNMENTS.agc
[INTERRUPT_LEAD_INS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INTERRUPT_LEAD_INS.agc
[T4RUPT_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/T4RUPT_PROGRAM.agc
[DOWNLINK_LISTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/DOWNLINK_LISTS.agc
[FRESH_START_AND_RESTART.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/FRESH_START_AND_RESTART.agc
[RESTART_TABLES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/RESTART_TABLES.agc
[SXTMARK.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/SXTMARK.agc
[EXTENDED_VERBS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/EXTENDED_VERBS.agc
[PINBALL_NOUN_TABLES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/PINBALL_NOUN_TABLES.agc
[CSM_GEOMETRY.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CSM_GEOMETRY.agc
[IMU_COMPENSATION_PACKAGE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/IMU_COMPENSATION_PACKAGE.agc
[PINBALL_GAME_BUTTONS_AND_LIGHTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/PINBALL_GAME_BUTTONS_AND_LIGHTS.agc
[R60_62.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/R60_62.agc
[ANGLFIND.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ANGLFIND.agc
[GIMBAL_LOCK_AVOIDANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/GIMBAL_LOCK_AVOIDANCE.agc
[KALCMANU_STEERING.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/KALCMANU_STEERING.agc
[SYSTEM_TEST_STANDARD_LEAD_INS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/SYSTEM_TEST_STANDARD_LEAD_INS.agc
[IMU_CALIBRATION_AND_ALIGNMENT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/IMU_CALIBRATION_AND_ALIGNMENT.agc
[GROUND_TRACKING_DETERMINATION_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/GROUND_TRACKING_DETERMINATION_PROGRAM.agc
[P34-35_P74-75.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P34-35_P74-75.agc
[R31.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/R31.agc
[P76.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P76.agc
[R30.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/R30.agc
[STABLE_ORBIT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/STABLE_ORBIT.agc
[P11.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P11.agc
[TPI_SEARCH.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TPI_SEARCH.agc
[P20-P25.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P20-P25.agc
[P30-P37.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P30-P37.agc
[P32-P33_P72-P73.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P32-P33_P72-P73.agc
[P40-P47.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P40-P47.agc
[P51-P53.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P51-P53.agc
[LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc
[P61-P67.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P61-P67.agc
[SERVICER207.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/SERVICER207.agc
[ENTRY_LEXICON.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ENTRY_LEXICON.agc
[REENTRY_CONTROL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/REENTRY_CONTROL.agc
[CM_BODY_ATTITUDE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CM_BODY_ATTITUDE.agc
[P37_P70.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/P37_P70.agc
[S-BAND_ANTENNA_FOR_CM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/S-BAND_ANTENNA_FOR_CM.agc
[LUNAR_LANDMARK_SELECTION_FOR_CM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/LUNAR_LANDMARK_SELECTION_FOR_CM.agc
[TVCINITIALIZE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCINITIALIZE.agc
[TVCEXECUTIVE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCEXECUTIVE.agc
[TVCMASSPROP.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCMASSPROP.agc
[TVCRESTARTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCRESTARTS.agc
[TVCDAPS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCDAPS.agc
[TVCSTROKETEST.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCSTROKETEST.agc
[TVCROLLDAP.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TVCROLLDAP.agc
[MYSUBS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/MYSUBS.agc
[RCS-CSM_DIGITAL_AUTOPILOT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/RCS-CSM_DIGITAL_AUTOPILOT.agc
[AUTOMATIC_MANEUVERS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/AUTOMATIC_MANEUVERS.agc
[RCS-CSM_DAP_EXECUTIVE_PROGRAMS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/RCS-CSM_DAP_EXECUTIVE_PROGRAMS.agc
[JET_SELECTION_LOGIC.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/JET_SELECTION_LOGIC.agc
[CM_ENTRY_DIGITAL_AUTOPILOT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CM_ENTRY_DIGITAL_AUTOPILOT.agc
[DOWN-TELEMETRY_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/DOWN-TELEMETRY_PROGRAM.agc
[INTER-BANK_COMMUNICATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INTER-BANK_COMMUNICATION.agc
[INTERPRETER.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INTERPRETER.agc
[FIXED_FIXED_CONSTANT_POOL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/FIXED_FIXED_CONSTANT_POOL.agc
[INTERPRETIVE_CONSTANTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INTERPRETIVE_CONSTANTS.agc
[SINGLE_PRECISION_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/SINGLE_PRECISION_SUBROUTINES.agc
[EXECUTIVE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/EXECUTIVE.agc
[WAITLIST.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/WAITLIST.agc
[LATITUDE_LONGITUDE_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/LATITUDE_LONGITUDE_SUBROUTINES.agc
[PLANETARY_INERTIAL_ORIENTATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/PLANETARY_INERTIAL_ORIENTATION.agc
[MEASUREMENT_INCORPORATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/MEASUREMENT_INCORPORATION.agc
[CONIC_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/CONIC_SUBROUTINES.agc
[INTEGRATION_INITIALIZATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INTEGRATION_INITIALIZATION.agc
[ORBITAL_INTEGRATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ORBITAL_INTEGRATION.agc
[INFLIGHT_ALIGNMENT_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/INFLIGHT_ALIGNMENT_ROUTINES.agc
[POWERED_FLIGHT_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/POWERED_FLIGHT_SUBROUTINES.agc
[TIME_OF_FREE_FALL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/TIME_OF_FREE_FALL.agc
[STAR_TABLES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/STAR_TABLES.agc
[AGC_BLOCK_TWO_SELF-CHECK.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/AGC_BLOCK_TWO_SELF-CHECK.agc
[PHASE_TABLE_MAINTENANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/PHASE_TABLE_MAINTENANCE.agc
[RESTARTS_ROUTINE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/RESTARTS_ROUTINE.agc
[IMU_MODE_SWITCHING_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/IMU_MODE_SWITCHING_ROUTINES.agc
[KEYRUPT_UPRUPT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/KEYRUPT_UPRUPT.agc
[DISPLAY_INTERFACE_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/DISPLAY_INTERFACE_ROUTINES.agc
[SERVICE_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/SERVICE_ROUTINES.agc
[ALARM_AND_ABORT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/ALARM_AND_ABORT.agc
[UPDATE_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/UPDATE_PROGRAM.agc
[RT8_OP_CODES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Comanche055/RT8_OP_CODES.agc
