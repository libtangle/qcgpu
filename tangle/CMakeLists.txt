# option(DISTRIBUTED "Use MPI" OFF)
# option(MULTITHREADED "Use OpenMP" OFF)

if (DISTRIBUTED)
    find_package(MPI REQUIRED)
    include_directories(${MPI_INCLUDE_PATH})
elseif (MULTITHREADED)
    find_package(OpenMP REQUIRED)
endif()


set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3")


add_subdirectory(src)
message("Tangle Source: ${TANGLE_SRC}")
add_library(tangle SHARED ${TANGLE_SRC})

# Header files & linking
target_include_directories(tangle PRIVATE src PUBLIC include)


if (DISTRIBUTED)
    target_link_libraries(tangle ${MPI_C_LIBRARIES})
    add_compile_definitions(_DISTRIBUTED)
elseif (MULTITHREADED)
    target_link_libraries(tangle PUBLIC OpenMP::OpenMP_CXX)
    add_compile_definitions(_MULTITHREADED)

endif()
