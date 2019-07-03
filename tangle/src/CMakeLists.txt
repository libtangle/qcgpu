

if (DISTRIBUTED)
    message("Dis")
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/implementation_mpi.c
        PARENT_SCOPE
    )
else()
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/implementation.c
        PARENT_SCOPE
    )
endif()