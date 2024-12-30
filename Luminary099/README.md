# Luminary099

The images (with suitable reduction in storage size and consequent reduction in image quality as well) are available online at www.ibiblio.org/apollo. If for some reason you find that the images are illegible, contact me at info@sandroid.org about getting access to the (much) higher-quality images which Paul actually created.

## Background

The contents of the "Luminary099" files, in general, are transcribed from a digital images created from a hardcopy of the program residing at the MIT Museum.  Many thanks to Debbie Douglas of the Museum, and to Paul Fjeld (who made the images).

Notations on this document read, in part:

```text
ASSEMBLE REVISION 001 OF AGC PROGRAM LMY99 BY NASA 2021112-061
16:27 JULY 14,1969
[Note that this is the date the hardcopy was made,
not the date of the program revision or the assembly.]
...
THIS LGC PROGRAM IS INTENDED FOR USE IN THE LM DURING THE MANNED
LUNAR LANDING MISSION OR ANY SUBSET THEREOF.
...
```

For organizational purposes RSB split the huge monolithic source code into smaller, more manageable chunks--i.e., into individual source files.  Those files are rejoined within `MAIN.agc` file as "includes".  It just makes it a little easier to work with.  The code chunks correspond to natural divisions into sub-programs.  In fact, these divisions are more-or-less specified by the source code itself.  Refer to the "TABLE OF SUBROUTINE LOG SECTIONS" at the very beginning of the file `ASSEMBLY_AND_OPERATION_INFORMATION.agc`.

It may be reasonably asked why tens of thousands of lines of source are joined by means of inclusion, rather than simply assembling the source files individually and then linking them to form the executable. The answer is that the original development team had no linker.  The builds were monolithic just like this.

There was a big emphasis on reusability of the code in the original project, apparently, but this reusability took the form of inserting your deck of punch-cards at the appropriate position in somebody else's deck of punch-cards. (Actually, I believe a tape-library method was used to avoid having to continually reload the card decks, but that doesn't change the basic principle.) So, indeed, the method of file-inclusion is a very fair representation of the methods used in the original development ... with the improvement, of course, that you no longer have to worry about dropping the card deck.  On the other hand, I wasn't there at the time, so I may have no idea what I'm talking about.

Finally, note that the original Apollo AGC assembler (called "YUL") is no longer available (as far as I can tell).  In fact, it was replaced by another assembler ("GAP") even before Apollo 11, but GAP is no more available than is YUL.  The replacement assembler yaYUL accepts a slightly different format for the source code from what YUL or GAP accepted, so the source code has been targeted for assembly with yaYUL.

What follows is simply a bunch of file-includes for the individual code chunks. I've marked the page numbers to make proof-reading easier.  Besides, the digital images of the assembly listing contains a lot of interesting tables (cross-referenced to page numbers) created by GAP, but not duplicated by yaYUL, so it's still valuable even if the source-files listed below are at hand.

## Source Code Index

*Derived from [MAIN.agc]*

| Source File                                   | Page Number |
| :-------------------------------------------- | :---------- |
| [ASSEMBLY_AND_OPERATION_INFORMATION.agc]      | 1-27        |
| [TAGS_FOR_RELATIVE_SETLOC.agc]                | 28-37       |
| [CONTROLLED_CONSTANTS.agc]                    | 38-53       |
| [INPUT_OUTPUT_CHANNEL_BIT_DESCRIPTIONS.agc]   | 54-60       |
| [FLAGWORD_ASSIGNMENTS.agc]                    | 61-88       |
| [ERASABLE_ASSIGNMENTS.agc]                    | 90-152      |
| [INTERRUPT_LEAD_INS.agc]                      | 153-154     |
| [T4RUPT_PROGRAM.agc]                          | 155-189     |
| [RCS_FAILURE_MONITOR.agc]                     | 190-192     |
| [DOWNLINK_LISTS.agc]                          | 193-205     |
| [AGS_INITIALIZATION.agc]                      | 206-210     |
| [FRESH_START_AND_RESTART.agc]                 | 211-237     |
| [RESTART_TABLES.agc]                          | 238-243     |
| [AOTMARK.agc]                                 | 244-261     |
| [EXTENDED_VERBS.agc]                          | 262-300     |
| [PINBALL_NOUN_TABLES.agc]                     | 301-319     |
| [LEM_GEOMETRY.agc]                            | 320-325     |
| [IMU_COMPENSATION_PACKAGE.agc]                | 326-337     |
| [R63.agc]                                     | 338-341     |
| [ATTITUDE_MANEUVER_ROUTINE.agc]               | 342-363     |
| [GIMBAL_LOCK_AVOIDANCE.agc]                   | 364         |
| [KALCMANU_STEERING.agc]                       | 365-369     |
| [SYSTEM_TEST_STANDARD_LEAD_INS.agc]           | 370-372     |
| [IMU_PERFORMANCE_TEST_2.agc]                  | 373-381     |
| [IMU_PERFORMANCE_TESTS_4.agc]                 | 382-389     |
| [PINBALL_GAME_BUTTONS_AND_LIGHTS.agc]         | 390-471     |
| [R60_62.agc]                                  | 472-485     |
| [S-BAND_ANTENNA_FOR_LM.agc]                   | 486-489     |
| [RADAR_LEADIN_ROUTINES.agc]                   | 490-491     |
| [P20-P25.agc]                                 | 492-614     |
| [P30_P37.agc]                                 | 615-617     |
| [P32-P35_P72-P75.agc]                         | 618-650     |
| [GENERAL_LAMBERT_AIMPOINT_GUIDANCE.agc]       | 651-653     |
| [GROUND_TRACKING_DETERMINATION_PROGRAM.agc]   | 654-657     |
| [P34-35_P74-75.agc]                           | 658-702     |
| [R31.agc]                                     | 703-708     |
| [P76.agc]                                     | 709-711     |
| [R30.agc]                                     | 712-722     |
| [STABLE_ORBIT.agc]                            | 723-730     |
| [BURN_BABY_BURN--MASTER_IGNITION_ROUTINE.agc] | 731-751     |
| [P40-P47.agc]                                 | 752-784     |
| [THE_LUNAR_LANDING.agc]                       | 785-792     |
| [THROTTLE_CONTROL_ROUTINES.agc]               | 793-797     |
| [LUNAR_LANDING_GUIDANCE_EQUATIONS.agc]        | 798-828     |
| [P70-P71.agc]                                 | 829-837     |
| [P12.agc]                                     | 838-842     |
| [ASCENT_GUIDANCE.agc]                         | 843-856     |
| [SERVICER.agc]                                | 857-897     |
| [LANDING_ANALOG_DISPLAYS.agc]                 | 898-907     |
| [FINDCDUW--GUIDAP_INTERFACE.agc]              | 908-925     |
| [P51-P53.agc]                                 | 926-983     |
| [LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc] | 984-987     |
| [DOWN_TELEMETRY_PROGRAM.agc]                  | 988-997     |
| [INTER-BANK_COMMUNICATION.agc]                | 998-1001    |
| [INTERPRETER.agc]                             | 1002-1094   |
| [FIXED_FIXED_CONSTANT_POOL.agc]               | 1095-1099   |
| [INTERPRETIVE_CONSTANT.agc]                   | 1100-1101   |
| [SINGLE_PRECISION_SUBROUTINES.agc]            | 1102        |
| [EXECUTIVE.agc]                               | 1103-1116   |
| [WAITLIST.agc]                                | 1117-1132   |
| [LATITUDE_LONGITUDE_SUBROUTINES.agc]          | 1133-1139   |
| [PLANETARY_INERTIAL_ORIENTATION.agc]          | 1140-1148   |
| [MEASUREMENT_INCORPORATION.agc]               | 1149-1158   |
| [CONIC_SUBROUTINES.agc]                       | 1159-1204   |
| [INTEGRATION_INITIALIZATION.agc]              | 1205-1226   |
| [ORBITAL_INTEGRATION.agc]                     | 1227-1248   |
| [INFLIGHT_ALIGNMENT_ROUTINES.agc]             | 1249-1258   |
| [POWERED_FLIGHT_SUBROUTINES.agc]              | 1259-1267   |
| [TIME_OF_FREE_FALL.agc]                       | 1268-1283   |
| [AGC_BLOCK_TWO_SELF_CHECK.agc]                | 1284-1293   |
| [PHASE_TABLE_MAINTENANCE.agc]                 | 1294-1302   |
| [RESTARTS_ROUTINE.agc]                        | 1303-1308   |
| [IMU_MODE_SWITCHING_ROUTINES.agc]             | 1309-1337   |
| [KEYRUPT_UPRUPT.agc]                          | 1338-1340   |
| [DISPLAY_INTERFACE_ROUTINES.agc]              | 1341-1373   |
| [SERVICE_ROUTINES.agc]                        | 1374-1380   |
| [ALARM_AND_ABORT.agc]                         | 1381-1385   |
| [UPDATE_PROGRAM.agc]                          | 1386-1396   |
| [RTB_OP_CODES.agc]                            | 1397-1402   |
| [T6-RUPT_PROGRAMS.agc]                        | 1403-1405   |
| [DAP_INTERFACE_SUBROUTINES.agc]               | 1406-1409   |
| [DAPIDLER_PROGRAM.agc]                        | 1410-1420   |
| [P-AXIS_RCS_AUTOPILOT.agc]                    | 1421-1441   |
| [Q_R-AXIS_RCS_AUTOPILOT.agc]                  | 1442-1459   |
| [TJET_LAW.agc]                                | 1460-1469   |
| [KALMAN_FILTER.agc]                           | 1470-1471   |
| [TRIM_GIMBAL_CONTROL_SYSTEM.agc]               | 1472-1484   |
| [AOSTASK_AND_AOSJOB.agc]                      | 1485-1506   |
| [SPS_BACK-UP_RCS_CONTROL.agc]                 | 1507-1510   |

### MISCELLANEOUS

| Source File          | Page Number   |
| :------------------- | :------------ |
| GAP-generated tables | 89, 1511-1743 |

[MAIN.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/MAIN.agc
[ASSEMBLY_AND_OPERATION_INFORMATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ASSEMBLY_AND_OPERATION_INFORMATION.agc
[TAGS_FOR_RELATIVE_SETLOC.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/TAGS_FOR_RELATIVE_SETLOC.agc
[CONTROLLED_CONSTANTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/CONTROLLED_CONSTANTS.agc
[INPUT_OUTPUT_CHANNEL_BIT_DESCRIPTIONS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INPUT_OUTPUT_CHANNEL_BIT_DESCRIPTIONS.agc
[FLAGWORD_ASSIGNMENTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/FLAGWORD_ASSIGNMENTS.agc
[ERASABLE_ASSIGNMENTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ERASABLE_ASSIGNMENTS.agc
[INTERRUPT_LEAD_INS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INTERRUPT_LEAD_INS.agc
[T4RUPT_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/T4RUPT_PROGRAM.agc
[RCS_FAILURE_MONITOR.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/RCS_FAILURE_MONITOR.agc
[DOWNLINK_LISTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/DOWNLINK_LISTS.agc
[AGS_INITIALIZATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/AGS_INITIALIZATION.agc
[FRESH_START_AND_RESTART.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/FRESH_START_AND_RESTART.agc
[RESTART_TABLES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/RESTART_TABLES.agc
[AOTMARK.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/AOTMARK.agc
[EXTENDED_VERBS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/EXTENDED_VERBS.agc
[PINBALL_NOUN_TABLES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/PINBALL_NOUN_TABLES.agc
[LEM_GEOMETRY.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/LEM_GEOMETRY.agc
[IMU_COMPENSATION_PACKAGE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/IMU_COMPENSATION_PACKAGE.agc
[R63.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/R63.agc
[ATTITUDE_MANEUVER_ROUTINE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ATTITUDE_MANEUVER_ROUTINE.agc
[GIMBAL_LOCK_AVOIDANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/GIMBAL_LOCK_AVOIDANCE.agc
[KALCMANU_STEERING.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/KALCMANU_STEERING.agc
[SYSTEM_TEST_STANDARD_LEAD_INS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/SYSTEM_TEST_STANDARD_LEAD_INS.agc
[IMU_PERFORMANCE_TEST_2.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/IMU_PERFORMANCE_TEST_2.agc
[IMU_PERFORMANCE_TESTS_4.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/IMU_PERFORMANCE_TESTS_4.agc
[PINBALL_GAME_BUTTONS_AND_LIGHTS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/PINBALL_GAME_BUTTONS_AND_LIGHTS.agc
[R60_62.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/R60_62.agc
[S-BAND_ANTENNA_FOR_LM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/S-BAND_ANTENNA_FOR_LM.agc
[RADAR_LEADIN_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/RADAR_LEADIN_ROUTINES.agc
[P20-P25.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P20-P25.agc
[P30_P37.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P30_P37.agc
[P32-P35_P72-P75.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P32-P35_P72-P75.agc
[GENERAL_LAMBERT_AIMPOINT_GUIDANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/GENERAL_LAMBERT_AIMPOINT_GUIDANCE.agc
[GROUND_TRACKING_DETERMINATION_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/GROUND_TRACKING_DETERMINATION_PROGRAM.agc
[P34-35_P74-75.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P34-35_P74-75.agc
[R31.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/R31.agc
[P76.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P76.agc
[R30.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/R30.agc
[STABLE_ORBIT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/STABLE_ORBIT.agc
[BURN_BABY_BURN--MASTER_IGNITION_ROUTINE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/BURN_BABY_BURN--MASTER_IGNITION_ROUTINE.agc
[P40-P47.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P40-P47.agc
[THE_LUNAR_LANDING.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/THE_LUNAR_LANDING.agc
[THROTTLE_CONTROL_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/THROTTLE_CONTROL_ROUTINES.agc
[LUNAR_LANDING_GUIDANCE_EQUATIONS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/LUNAR_LANDING_GUIDANCE_EQUATIONS.agc
[P70-P71.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P70-P71.agc
[P12.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P12.agc
[ASCENT_GUIDANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ASCENT_GUIDANCE.agc
[SERVICER.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/SERVICER.agc
[LANDING_ANALOG_DISPLAYS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/LANDING_ANALOG_DISPLAYS.agc
[FINDCDUW--GUIDAP_INTERFACE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/FINDCDUW--GUIDAP_INTERFACE.agc
[P51-P53.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P51-P53.agc
[LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/LUNAR_AND_SOLAR_EPHEMERIDES_SUBROUTINES.agc
[DOWN_TELEMETRY_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/DOWN_TELEMETRY_PROGRAM.agc
[INTER-BANK_COMMUNICATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INTER-BANK_COMMUNICATION.agc
[INTERPRETER.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INTERPRETER.agc
[FIXED_FIXED_CONSTANT_POOL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/FIXED_FIXED_CONSTANT_POOL.agc
[INTERPRETIVE_CONSTANT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INTERPRETIVE_CONSTANT.agc
[SINGLE_PRECISION_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/SINGLE_PRECISION_SUBROUTINES.agc
[EXECUTIVE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/EXECUTIVE.agc
[WAITLIST.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/WAITLIST.agc
[LATITUDE_LONGITUDE_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/LATITUDE_LONGITUDE_SUBROUTINES.agc
[PLANETARY_INERTIAL_ORIENTATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/PLANETARY_INERTIAL_ORIENTATION.agc
[MEASUREMENT_INCORPORATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/MEASUREMENT_INCORPORATION.agc
[CONIC_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/CONIC_SUBROUTINES.agc
[INTEGRATION_INITIALIZATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INTEGRATION_INITIALIZATION.agc
[ORBITAL_INTEGRATION.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ORBITAL_INTEGRATION.agc
[INFLIGHT_ALIGNMENT_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/INFLIGHT_ALIGNMENT_ROUTINES.agc
[POWERED_FLIGHT_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/POWERED_FLIGHT_SUBROUTINES.agc
[TIME_OF_FREE_FALL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/TIME_OF_FREE_FALL.agc
[AGC_BLOCK_TWO_SELF_CHECK.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/AGC_BLOCK_TWO_SELF_CHECK.agc
[PHASE_TABLE_MAINTENANCE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/PHASE_TABLE_MAINTENANCE.agc
[RESTARTS_ROUTINE.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/RESTARTS_ROUTINE.agc
[IMU_MODE_SWITCHING_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/IMU_MODE_SWITCHING_ROUTINES.agc
[KEYRUPT_UPRUPT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/KEYRUPT_UPRUPT.agc
[DISPLAY_INTERFACE_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/DISPLAY_INTERFACE_ROUTINES.agc
[SERVICE_ROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/SERVICE_ROUTINES.agc
[ALARM_AND_ABORT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/ALARM_AND_ABORT.agc
[UPDATE_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/UPDATE_PROGRAM.agc
[RTB_OP_CODES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/RTB_OP_CODES.agc
[T6-RUPT_PROGRAMS.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/T6-RUPT_PROGRAMS.agc
[DAP_INTERFACE_SUBROUTINES.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/DAP_INTERFACE_SUBROUTINES.agc
[DAPIDLER_PROGRAM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/DAPIDLER_PROGRAM.agc
[P-AXIS_RCS_AUTOPILOT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/P-AXIS_RCS_AUTOPILOT.agc
[Q_R-AXIS_RCS_AUTOPILOT.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/Q_R-AXIS_RCS_AUTOPILOT.agc
[TJET_LAW.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/TJET_LAW.agc
[KALMAN_FILTER.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/KALMAN_FILTER.agc
[TRIM_GIMBAL_CONTROL_SYSTEM.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/TRIM_GIMBAL_CONTROL_SYSTEM.agc
[AOSTASK_AND_AOSJOB.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/AOSTASK_AND_AOSJOB.agc
[SPS_BACK-UP_RCS_CONTROL.agc]:https://github.com/chrislgarry/Apollo-11/blob/master/Luminary099/SPS_BACK-UP_RCS_CONTROL.agc
