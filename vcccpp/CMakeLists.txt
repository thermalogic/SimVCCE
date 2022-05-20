cmake_minimum_required(VERSION 3.0.0)
project(vcc VERSION 0.1.0)

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY "../bin")
set(CMAKE_CXX_FLAGS "-DCOOLPROP_LIB")

include_directories(include) 
link_directories(bin)
aux_source_directory(./src/ SRC)

add_executable(demovcc demovcc.cpp ${SRC})
target_link_libraries(demovcc CoolProp)

