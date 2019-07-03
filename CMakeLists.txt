cmake_minimum_required(VERSION 3.9)

project(tangle)

find_package(MPI REQUIRED)

# Add headers
include_directories(include)

file(GLOB SOURCES "src/*.c")

add_executable(main ${SOURCES})

target_link_libraries(main PRIVATE MPI::MPI_C)