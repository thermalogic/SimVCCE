# SimVCCE in C++

The vapor-compression refrigeration cycle steady-state simulator for education in C++

## Prerequisites：The shared library of CoolProp

* `CoolProp.dll` and `CoolProp.lib` are the libraries builded with MSVC 

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

#### Visual C++ on Windows or GCC on Linux 

in the terminal of `./build`

```bash
cmake ..  
```

#### TDM-GCC-64 on Windows

in the terminal of `./build`:

```bash
cmake .. -G "MinGW Makefiles" -D"CMAKE_MAKE_PROGRAM:PATH=C:/TDM-GCC-64/bin/make.exe" 
```
