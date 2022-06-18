ifneq ($(OS),Windows_NT)
	UNAME_S := $(shell uname -s)
endif

SRC=./src/
BIN=./bin/
INC=./include/
LIBFLAGS=-L./bin -lCoolProp
CFLAGES=-std=c++17 -O3 -DCOOLPROP_LIB 

SRCS=./demovcc.cpp \
	$(SRC)/vcc.cpp \
	$(SRC)/port.cpp \
	$(SRC)/connector.cpp \
	$(SRC)/compressor.cpp \
	$(SRC)/condenser.cpp \
	$(SRC)/evaporator.cpp \
	$(SRC)/expansionvalve.cpp 

OUTFILE=$(BIN)demovcc

all: $(OUTFILE)
	$(OUTFILE)

$(OUTFILE): $(SRCS)  
	g++ $(CFLAGES) -o $@ $^ -I$(INC) -I$(SRC) $(LIBFLAGS)

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
