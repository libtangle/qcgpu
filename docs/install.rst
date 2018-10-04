============
Installation
============

Prerequisites
-------------

To use QCGPU you will need to be using `Python 2.7 or later <https://www.python.org/downloads/>`_.
You will also need to ensure that you have an `OpenCL <https://www.khronos.org/opencl/>`_ implementation installed. 
This is done by default on MacOS, but you shuld check that ``clinfo`` or some other diagnostic command will run.

You can also use `Anaconda 3 <https://www.continuum.io/downloads>`_, which will have many of the required dependencies already installed.

Installing from PyPI
--------------------

This library is distributed on `PyPI <https://pypi.python.org/pypi/qcgpu>`_ and can be installed using pip:

.. code:: sh

   $ pip install qcgpu

If you run into any issues, you should try installing from source.

Installing from Source
----------------------

You can install QCGPU from the source. First, clone the repository off
GitHub:

.. code:: sh

   $ git clone https://github.com/qcgpu/qcgpu

Then you will need to ``cd`` into the directory, and install the
requirements.

.. code:: sh

   $ pip install -r requirements.txt

And finally you can install:

.. code:: sh

   $ python setup.py install
