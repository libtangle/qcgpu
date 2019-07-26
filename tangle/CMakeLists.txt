find_package(PythonExtensions REQUIRED)
find_package(Cython REQUIRED)


add_cython_target(_tangle.pyx CXX)
add_library(_tangle MODULE ${_tangle})
target_include_directories(_tangle PRIVATE ${TANGLE_SRC_DIR})
python_extension_module(_tangle)

install(TARGETS _tangle LIBRARY DESTINATION tangle)