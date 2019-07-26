find_package(PythonExtensions REQUIRED)

add_library(_tangle MODULE tangle/_tangle.cxx)
python_extension_module(_tangle)
install(TARGETS _tangle LIBRARY DESTINATION tangle)
