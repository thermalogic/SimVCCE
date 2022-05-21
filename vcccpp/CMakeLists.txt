cmake_minimum_required(VERSION 3.0.0)
project(vcc VERSION 0.1.0)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "../bin")

if(MSVC)
   set(CMAKE_CXX_FLAGS "-DCOOLPROP_LIB /std:c++17 /EHsc /Wv:18" )
else()
   set(CMAKE_CXX_FLAGS "-DCOOLPROP_LIB -std=c++17")
endif()

include_directories(include) 
link_directories(bin)
aux_source_directory(./src/ SRC)

add_executable(demovcc ${SRC} demovcc.cpp)
target_link_libraries(demovcc CoolProp)
