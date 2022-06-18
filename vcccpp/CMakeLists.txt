cmake_minimum_required(VERSION 3.0.0)
project(vcc VERSION 0.1.0)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "../bin")

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_CXX_FLAGS "-DCOOLPROP_LIB")
if(MSVC)
   set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} /EHsc /Wv:18")
endif()

include_directories(include src)  
link_directories(bin)
aux_source_directory(./src/ SRC)

add_executable(demovcc ${SRC} demovcc.cpp)
target_link_libraries(demovcc CoolProp)
