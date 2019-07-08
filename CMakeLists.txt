cmake_minimum_required(VERSION 3.9)

project(tangle_project)

# EDIT HERE 

set(USER_SOURCE "example.c")
set(OUTPUT_EXE "example")

option(DISTRIBUTED "Use MPI" OFF)
option(MULTITHREADED "Use OpenMP" ON)

# END EDIT

if (NOT DEFINED ${TANGLE_LIB_PATH})
    set(TANGLE_DIR "tangle")
    add_subdirectory(${TANGLE_DIR})
    set(TANGLE_LIB_PATH "${CMAKE_CURRENT_BINARY_DIR}/${TANGLE_DIR}")
    set(TANGLE_LIB_EXACT "${TANGLE_LIB_PATH}/libtangle.so")
endif()


add_executable(${OUTPUT_EXE} ${USER_SOURCE})
target_link_libraries(${OUTPUT_EXE} tangle)