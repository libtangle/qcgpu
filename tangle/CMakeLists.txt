# Builds Tangle as a shared library, libtangle.so

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

# Header files

target_include_directories(tangle PRIVATE src PUBLIC include)

# ------------------------------
# ---- DEFINITIONS
# ------------------------------

add_compile_definitions(_DO_OTHER)