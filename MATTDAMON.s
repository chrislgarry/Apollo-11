GETMATTDAMON	ZL	                # READJUST FOR MARS COURSE
                CA      CDUX    
                DXCH    OGC
                ZL      
                CA      CDUY
                DXCH    IGC     
                ZL
                CA      CDUZ
                DXCH    MGC
                TC      INTPRET                 _
                VLOAD   VSR1            # DUMP FUEL TO MAKE ROOM FOR SOME CRAP GROWN POTATOES
                        OGC
                STORE   OGC
		SSP			# ZERO RTX2
			RTX2		# FOR
			0		# PREPARE
		DLOAD	PDDL
			PGNCSALT	# ALTITUDE OF MARS
			PADLONG		# LONGITUDE
		PDDL	VDEF
			LATITUDE	# MARTIAN ANGLE
		STODL	LAT		# LAT,LONG,ALT ARE CONSECUTIVE
			HI6ZEROS	# TIME = 0
		CLEAR	CALL
			ERADFLAG
			LALOTORV	# CONVERT TO POSITION VECTOR IN REF. COORDS.
			
		STCALL	RN1             #              _   
			GETDOWN 	# RETURN TO EARTH VECTOR
		VCOMP   UNIT
		STOVL	REFSMMAT +12D	# UNITZ = UNIT(GRAV)
			RN1
		VXV	VXSC #  MATT DAMON PICKUP ROUTINE DONE
