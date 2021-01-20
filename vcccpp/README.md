# SimVCCE in C++

The vapor-compression refrigeration cycle steady-state simulator for education in C++

## Makefiles

1. `makefile`: Install CoolProp, building and run

2. `makelib.mk`: Building the SimVCCE shared Library(`libvcc.so/dll`)

3. `makelibapp.mk`: Building and run the client of the SimVCCE shared Library

## Prerequisites：The shared library of CoolProp

* `libCoolProp.dll` is the library builded with tdm64-gcc-9.2.0 on Windows 

* `libCoolProp.so.6.4.1`  is the library builded with GCC 9.3 and glibc2.29 above on Linux

## Install the CoolProp on Linux 

type `sudo make install` in the terminal of `./vcccpp` on Linux

```bash
$sudo make install
```

## Build and Run

type `make` in the terminal of `./vcccpp`

```bash
>make
```
