
LIBFLAGS=-L./bin -lCoolProp
ifeq ($(OS),Windows_NT)
    LIBNAME =./bin/libvcc.dll
else
	UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        LIBNAME =./bin/libvcc.so
    endif
endif

CFLAGES=-std=c++17 -O3 -DCOOLPROP_LIB -fPIC -c 

OBJDIR=./obj/
SRCDIR=./src/

SRCS=$(wildcard $(SRCDIR)*.cpp)

OBJS=$(patsubst %.cpp,$(OBJDIR)%.o,$(notdir $(SRCS)))

all:$(LIBNAME)

$(LIBNAME):$(OBJS)
	g++ -shared  -o $@ $^  $(LIBFLAGS)

$(OBJDIR)%.o:$(SRCDIR)%.cpp 
	g++ -o $@ $(CFLAGES)  $^  -I./include $(LIBFLAGS)

ifneq ($(OS),Windows_NT)
    ifeq ($(UNAME_S),Linux)
clean:
	rm -f  $(OBJDIR)*.o
  	endif
else
clean:
	del .\obj\*.o
endif	   
