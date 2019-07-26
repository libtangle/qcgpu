from skbuild import setup

with open("README.md", "r") as fh:
    long_description = fh.read()

setup(
    name="tangle",
    version="1.0.0",
    author="Adam Kelly",
    author_email="adamkelly2201@gmail.com",
    description="High Performance Tools for Quantum Computation",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://libtangle.com",
    packages=['tangle'],
    classifiers=[
        "Environment :: Console",
        "License :: OSI Approved :: MIT License",
        "Intended Audience :: Developers",
        "Intended Audience :: Science/Research",
        "Operating System :: Microsoft :: Windows",
        "Operating System :: MacOS",
        "Operating System :: POSIX :: Linux",
        "Programming Language :: C++",
        "Programming Language :: Python :: 3.5",
        "Programming Language :: Python :: 3.6",
        "Topic :: Scientific/Engineering",
    ],
    install_requires=['scikit-build']
)