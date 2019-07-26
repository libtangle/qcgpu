find_package(PythonExtensions REQUIRED)

add_library(_tangle MODULE _tangle.cpp)
python_extension_module(_tangle)
install(TARGETS _tangle LIBRARY DESTINATION tangle)
