# option(DISTRIBUTED "Use MPI" OFF)


if (DISTRIBUTED)
    find_package(MPI REQUIRED)
    include_directories(${MPI_INCLUDE_PATH})
endif()

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3")


add_subdirectory(src)
message("Tangle Source: ${TANGLE_SRC}")
add_library(tangle SHARED ${TANGLE_SRC})

# Header files & linking
target_include_directories(tangle PRIVATE src PUBLIC include)
target_link_libraries(tangle ${MPI_C_LIBRARIES})


if (DISTRIBUTED)
    add_compile_definitions(_DISTRIBUTED)
endif()
