if (DISTRIBUTED)
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/impl_mpi.c
        PARENT_SCOPE
    )
else()
    set(TANGLE_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/impl.c
        PARENT_SCOPE
    )
endif()