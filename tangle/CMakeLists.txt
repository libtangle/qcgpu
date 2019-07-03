# Builds Tangle as a shared library, libtangle.so

# ------------------------------
# ---- OPTIONS
# ------------------------------

option(DISTRIBUTED "Use MPI" ON)

# ------------------------------
# ---- FIND PACKAGES
# ------------------------------

if (DISTRIBUTED)
    find_package(MPI REQUIRED)
    include_directories(${MPI_INCLUDE_PATH})
endif()

# ------------------------------
# ---- COMPILER FLAGS
# ------------------------------

set(CMAKE_C_FLAGS "${CMAKE_C_FLAGS} -O3")

# ------------------------------
# ---- BUILD LIBRARY
# ------------------------------

add_subdirectory(src)
message("Tangle Source: ${TANGLE_SRC}")
add_library(tangle SHARED ${TANGLE_SRC})

# Header files & linking
target_include_directories(tangle PRIVATE src PUBLIC include)
target_link_libraries(tangle ${MPI_C_LIBRARIES})

# ------------------------------
# ---- DEFINITIONS
# ------------------------------

if (DISTRIBUTED)
    add_compile_definitions(_DISTRIBUTED)
endif()
