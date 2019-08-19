if (DISTRIBUTED)
    find_package(MPI REQUIRED)
elseif (MULTITHREADED)
    find_package(OpenMP REQUIRED)
endif()


add_subdirectory(src)
message("Tangle Source: ${TANGLE_SRC}")
message("Tangle Platform Specific Source: ${TANGLE_PLATFORM_SRC}")


add_library(tangle STATIC ${TANGLE_SRC} ${TANGLE_PLATFORM_SRC})
target_include_directories(tangle PRIVATE src PUBLIC include)


if (DISTRIBUTED)
    include_directories(${MPI_INCLUDE_PATH})
    target_link_libraries(tangle ${MPI_C_LIBRARIES})
elseif (MULTITHREADED)
    target_link_libraries(tangle PUBLIC OpenMP::OpenMP_C)
endif()

target_link_libraries(tangle m)