NVER:=\\\"2016-08-14-working\\\"
DATE:=`date +%Y%m%d`

# List of mission software directories to be built.
SUBDIRS = Luminary099 Comanche055

.PHONY: default
default: all

.PHONY: all 
all: $(SUBDIRS)

.PHONY: ${SUBDIRS}
$(SUBDIRS): 
	make -C $@

clean:
	for subdir in $(SUBDIRS) ; do make -C $$subdir clean ; done

.PHONY: corediffs
corediffs: 
	for subdir in $(SUBDIRS) ; do make -C $$subdir corediff.txt ; done

