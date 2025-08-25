# Simple program to add two numbers
# and store the result.

START   CA    NUM1      # Load NUM1 into the accumulator
        AD    NUM2      # Add NUM2 to the accumulator
        TS    RESULT    # Store the result in RESULT
        TCF   END       # Jump to END

NUM1    DEC   10        # A decimal constant
NUM2    DEC   5         # Another decimal constant
RESULT  ERASE 1         # A variable to store the result
END     TCF   END       # Infinite loop
