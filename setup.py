import setuptools

with open("README.md", "r") as fh:
    long_description = fh.read()

setuptools.setup(
    name="qcgpu",
    version="0.0.4",
    author="Adam Kelly",
    author_email="adamkelly2201@gmail.com",
    description="An OpenCL based quantum computer simulator",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/qcgpu/qcgpu",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    setup_requires=['pytest-runner'],
    install_requires=['mako', 'pyopencl', 'pybind11', 'numpy'],
    tests_require=["pytest"]
)
