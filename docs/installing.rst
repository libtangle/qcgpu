.. _installing:

Installing
==========

There are a few things you have to do to install QCGPU.

Prerequisites
-------------

-  `OpenCL`_ (Ensure that an OpenCL implementation is installed for your
   platform and that ``clinfo`` or some other diagnostic command will
   run).
-  `Python`_ (Version 2.7 or later).

Installing from PyPI
--------------------

This library is distributed on `PyPI`_ and can be installed using pip:

.. code:: bash

   $ pip install qcgpu

If you run into any issues, you should try installing from source.

Installing from Source
----------------------

You can install QCGPU from the source. First, clone the repository off
GitHub:

.. code:: bash

   $ git clone https://github.com/qcgpu/qcgpu

Then you will need to ``cd`` into the directory, and install the
requirements.

.. code:: bash

   $ pip install -r requirements.txt

And finally you can install:

.. code:: bash

   $ python setup.py install

.. _OpenCL: https://www.khronos.org/opencl/
.. _Python: https://www.python.org/
.. _PyPI: https://pypi.python.org/pypi/qcgpu