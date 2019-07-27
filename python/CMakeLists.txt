find_package(PythonExtensions REQUIRED)
find_package(Cython REQUIRED)

add_cython_target(pytangle pytangle.pyx C)
add_library(pytangle MODULE ${pytangle})

target_link_libraries(pytangle tangle)
target_include_directories(pytangle PUBLIC "${CMAKE_SOURCE_DIR}/tangle/include")

python_extension_module(pytangle)
install(TARGETS pytangle LIBRARY DESTINATION python)