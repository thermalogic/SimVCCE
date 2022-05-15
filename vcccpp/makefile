ifneq ($(OS),Windows_NT)
	UNAME_S := $(shell uname -s)
endif

BIN=./bin/
INC=./include/
LIBFLAGS=-L./bin -lCoolProp
CFLAGES=-std=c++17 -O3 -DCOOLPROP_LIB 

SRCS=./demovcc.cpp \
	./src/vcc.cpp \
	./src/port.cpp \
	./src/connector.cpp \
	./src/compressor.cpp \
	./src/condenser.cpp \
	./src/evaporator.cpp \
	./src/expansionvalve.cpp 

OUTFILE=$(BIN)demovcc

all: $(OUTFILE)
	$(OUTFILE)

$(OUTFILE): $(SRCS)  
	g++ $(CFLAGES) -o $@ $^ -I$(INC) $(LIBFLAGS)

run:
	$(OUTFILE)

ifneq ($(OS),Windows_NT)
    ifeq ($(UNAME_S),Linux)
        SONAME=libCoolProp.so.6
        REAL_NAME=libCoolProp.so.6.4.1
        LINKER_NAME=libCoolProp.so
install:
	cp $(BIN)$(REAL_NAME) /usr/lib/
	ln -sf $(REAL_NAME)  /usr/lib/$(SONAME)
	ln -sf $(SONAME)  /usr/lib/$(LINKER_NAME)
  	endif
else
install:
	@echo "Install for Linux Only!"
endif	
