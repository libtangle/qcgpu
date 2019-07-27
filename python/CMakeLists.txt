find_package(PythonExtensions REQUIRED)
find_package(Cython REQUIRED)

set(TANGLE_SRC_DIR "${PROJECT_SOURCE_DIR}/tangle/include")

add_cython_target(_tangle.pyx C)
add_library(_tangle MODULE ${_tangle})
target_link_libraries(_tangle libtangle)
# target_include_directories(_tangle PRIVATE ${TANGLE_SRC_DIR})
python_extension_module(_tangle)

message("ECO FROM MESSAAGE")


install(TARGETS _tangle LIBRARY DESTINATION tangle)