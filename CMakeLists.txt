cmake_minimum_required(VERSION 3.9)
project(tangle)

# Source for the command line tool
set(CLI_MAIN "${PROJECT_SOURCE_DIR}/extras/simulator_cli.cpp")

if(SKBUILD) # Using Python
    add_subdirectory(tangle)
else() # Using Command Line Tool
    add_subdirectory(extras)
endif()