cmake_minimum_required(VERSION 3.0)

project(tangle_project)

# ------------------------------
# ---- USER OPTIONS
# ------------------------------

set(USER_SOURCE "example.c")
set(OUTPUT_EXE "example")

# ------------------------------
# ---- TANGLE LIBRARY
# ------------------------------

if (NOT DEFINED ${TANGLE_LIB_PATH})
    # Build libtangle.so
    set(TANGLE_DIR "tangle")
    add_subdirectory(${TANGLE_DIR})
    set(TANGLE_LIB_PATH "${CMAKE_CURRENT_BINARY_DIR}/${TANGLE_DIR}")
    set(TANGLE_LIB_EXACT "${TANGLE_LIB_PATH}/libtangle.so")
endif()

# ------------------------------
# ---- USER EXECUTABLE
# ------------------------------

message(STATUS "Compiling ${USER_SOURCE} to executable ${OUTPUT_EXE}")

# Create user executable
add_executable(${OUTPUT_EXE} ${USER_SOURCE})

# Link libraries to user executable, including Tangle
target_link_libraries(${OUTPUT_EXE} tangle)