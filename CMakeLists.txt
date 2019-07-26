cmake_minimum_required(VERSION 3.9)
project(tangle)

set(TANGLE_SRC_DIR "${PROJECT_SOURCE_DIR}/src")
set(CLI_MAIN "${PROJECT_SOURCE_DIR}/extras/simulator_cli.cpp")

if(SKBUILD) # Using Python
    add_subdirectory(tangle)
else() # Using Command Line Tool
    add_subdirectory(extras)
endif()