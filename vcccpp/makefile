ifneq ($(OS),Windows_NT)
	UNAME_S := $(shell uname -s)
endif

SRC=./src/
BIN=./bin/
INC=./include/
LIBFLAGS=-L./bin -lCoolProp
CFLAGS=-std=c++17 -O3 -DCOOLPROP_LIB 

SRCS=$(SRC)/vcc.cpp \
	$(SRC)/core/port.cpp \
	$(SRC)/core/connector.cpp \
	$(SRC)/components/compressor.cpp \
	$(SRC)/components/condenser.cpp \
	$(SRC)/components/evaporator.cpp \
	$(SRC)/components/expansionvalve.cpp 

OUTFILE=$(BIN)/demovcc

all: $(OUTFILE)
	@echo "Compiled successfully!"
	@echo "Testing..."
	$(OUTFILE)

$(OUTFILE): demovcc.cpp $(SRCS)
	g++ $(CFLAGS) -o $@ $^ $(LIBFLAGS) -I$(INC) -I$(SRC)

ifneq ($(OS),Windows_NT)
    ifeq ($(UNAME_S),Linux)
SONAME=libCoolProp.so.6
REAL_NAME=libCoolProp.so.6.4.1
LINKER_NAME=libCoolProp.so
install:
	cp $(BIN)$(REAL_NAME) /usr/lib/
	ln -sf $(REAL_NAME) /usr/lib/$(SONAME)
	ln -sf $(SONAME) /usr/lib/$(LINKER_NAME)
    endif
else
install:
	@echo "Install for Linux Only!"
endif	

clean:
	rm -f $(OUTFILE)
