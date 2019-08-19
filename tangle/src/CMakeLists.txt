# Global Code
set(TANGLE_SRC
    ${CMAKE_CURRENT_SOURCE_DIR}/gates.c
    PARENT_SCOPE
)

# Platform Specific Code
if (DISTRIBUTED)
    set(TANGLE_PLATFORM_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/distributed/communication.c
        ${CMAKE_CURRENT_SOURCE_DIR}/distributed/environment.c
        ${CMAKE_CURRENT_SOURCE_DIR}/distributed/state.c
        ${CMAKE_CURRENT_SOURCE_DIR}/distributed/gate_application.c
        PARENT_SCOPE
    )
elseif (MULTITHREADED)
    set(TANGLE_PLATFORM_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/omp.c
        PARENT_SCOPE
    )
else()
    set(TANGLE_PLATFORM_SRC
        ${CMAKE_CURRENT_SOURCE_DIR}/cpu.c
        PARENT_SCOPE
    )
endif()