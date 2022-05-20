# SimVCCE in C++

The vapor-compression refrigeration cycle steady-state simulator for education in C++

## Prerequisites：The shared library of CoolProp

* `libCoolProp.dll` is the library builded with tdm64-gcc-9.2.0 on Windows 

* `libCoolProp.so.6.4.1`  is the library builded with GCC 9.3 and glibc2.29 above on Linux

## Install the CoolProp on Linux 

type `sudo make install` in the terminal of `./vcccpp` on Linux

```bash
$sudo make install
```

## Build and Run

### make

type `make` in the terminal of `./vcccpp`

```bash
>make
```
### cmake

### TDM-GCC-64 on Windows

in terminal of build:

```bash
cmake .. -G "MinGW Makefiles" -D"CMAKE_MAKE_PROGRAM:PATH=C:/TDM-GCC-64/bin/make.exe" 
```

### GCC on Linux 

in terminal of build:

```bash
cmake ..  
```