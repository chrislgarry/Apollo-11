SOURCE:=$(wildcard *.agc)
TARGETS=${BASENAME}.bin ${BASENAME}.lst 

default: $(TARGETS)

${BASENAME}.bin ${BASENAME}.lst: ${SOURCE}
	${VIRTUALAGC}/yaYUL/yaYUL --unpound-page --html MAIN.agc >MAIN.agc.lst
	diff -s MAIN.agc.bin ${VIRTUALAGC}/${BASENAME}/${BASENAME}.bin
	mv MAIN.agc.bin ${BASENAME}.bin
	mv MAIN.agc.lst ${BASENAME}.lst
	mv MAIN.agc.symtab ${BASENAME}.symtab

corediff.txt: ${BASENAME}.bin ${BASENAME}.lst
	python ${VIRTUALAGC}/Tools/ropediff.py -p -c -a -o $@ ${VIRTUALAGC}/${BASENAME}/${BASENAME}.bin ${BASENAME}.bin

clean:
	rm -f *.lst *~ *.bin *.symtab oct2bin.bin* *.agc.html
