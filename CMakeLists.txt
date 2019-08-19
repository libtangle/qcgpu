cmake_minimum_required(VERSION 3.9)
project(tangle_project)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=c++11")

option(DISTRIBUTED "Use MPI" ON)
option(MULTITHREADED "Use OpenMP" OFF)


if (NOT DEFINED ${TANGLE_LIB_PATH})
    set(TANGLE_DIR "tangle")
    add_subdirectory(${TANGLE_DIR})
    set(TANGLE_LIB_PATH "${CMAKE_SOURCE_DIR}/${TANGLE_DIR}")
    set(TANGLE_LIB_EXACT "${TANGLE_LIB_PATH}/libtangle.so")
endif()


set(CLI_SOURCE "standalone/cli.c")
set(OUTPUT_EXE "tangle-cli")

if(SKBUILD)
    add_subdirectory(python)
else()
    add_executable(tangle_cli ${CLI_SOURCE})
    set_target_properties(tangle_cli PROPERTIES OUTPUT_NAME ${OUTPUT_EXE})
    target_link_libraries(tangle_cli tangle)

    if (DISTRIBUTED)
        find_package(MPI REQUIRED)
        include_directories(${MPI_INCLUDE_PATH})
        target_link_libraries(tangle_cli ${MPI_C_LIBRARIES})
    elseif (MULTITHREADED)
        find_package(OpenMP REQUIRED)
        target_link_libraries(tangle_cli OpenMP::OpenMP_C)
    endif()
endif()
