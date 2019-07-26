find_package(PythonExtensions REQUIRED)
find_package(Cython REQUIRED)


# add_library(_tangle MODULE _tangle.cpp)
# python_extension_module(_tangle)
# install(TARGETS _tangle LIBRARY DESTINATION tangle)

# add_cython_target(${PROJECT_SOURCE_DIR}/tangle/_tangle.pyx)
add_cython_target(_tangle.pyx CXX)
add_library(_tangle MODULE ${_tangle})
python_extension_module(_tangle)

install(TARGETS _tangle LIBRARY DESTINATION tangle)