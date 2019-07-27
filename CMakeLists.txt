cmake_minimum_required(VERSION 3.9)

project(tangle_project)

# EDIT HERE 

option(DISTRIBUTED "Use MPI" OFF)
option(MULTITHREADED "Use OpenMP" ON)

# END EDIT

if (NOT DEFINED ${TANGLE_LIB_PATH})
    set(TANGLE_DIR "tangle")
    add_subdirectory(${TANGLE_DIR})
    set(TANGLE_LIB_PATH "${CMAKE_CURRENT_BINARY_DIR}/${TANGLE_DIR}")
    set(TANGLE_LIB_EXACT "${TANGLE_LIB_PATH}/libtangle.so")
endif()


# STANDALONE SIMULATOR

set(USER_SOURCE "standalone/cli.c")
set(OUTPUT_EXE "tangle-cli")

if(SKBUILD) # Using Python
    add_subdirectory(python)
else() # Using Command Line Tool
    add_executable(${OUTPUT_EXE} ${USER_SOURCE})
    target_link_libraries(${OUTPUT_EXE} libtangle)

    if (DISTRIBUTED)
    find_package(MPI REQUIRED)
    include_directories(${MPI_INCLUDE_PATH})

    target_link_libraries(${OUTPUT_EXE} ${MPI_C_LIBRARIES})
    add_compile_definitions(_DISTRIBUTED)
    elseif (MULTITHREADED)
    find_package(OpenMP REQUIRED)

    target_link_libraries(${OUTPUT_EXE} OpenMP::OpenMP_C)
    add_compile_definitions(_MULTITHREADED)
    endif()
endif()
