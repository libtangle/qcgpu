if (DISTRIBUTED)
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/mpi.c
        PARENT_SCOPE
    )
elseif (MULTITHREADED)
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/omp.c
        PARENT_SCOPE
    )
else()
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/cpu.c
        PARENT_SCOPE
    )
endif()