cmake_minimum_required(VERSION 3.5)
project(osd-capi)

set(CMAKE_CXX_STANDARD 14)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(osd-capi STATIC 
    far/stencil_table.cpp
    far/stencil_table_factory.cpp
    far/topology_refiner_factory.cpp
    far/topology_refiner.cpp
    far/topology_level.cpp
    osd/cpu_vertex_buffer.cpp
    osd/cpu_evaluator.cpp
)

target_include_directories(osd-capi PUBLIC ${INC_OSD})
target_link_libraries(osd-capi PUBLIC ${LIB_OSD}/lib/libosdCPU.so ${LIB_OSD}/lib/libosdGPU.so)
install(TARGETS osd-capi DESTINATION ${CMAKE_INSTALL_PREFIX})