# Notes

> This document is to keep track of various ideas, plans ect. relating
> to the development of QCGPU.

## Simulators

At a bare minimum, a quantum computer simulator must have the following parts:

* Ability to create a quantum register with a given numebr of qubits
* Ability to apply controlled gates, or at minimum the controlled pauli-x gate (CNOT)
* Ability to apply single qubit gates, which combined with the CNOT form a universal set of gates.
* Ability to measure single qubits in the register and collapse their state into the measured state.

Other functionality can be added which will make the simulator more useful, which is discussed later

## Alternative Simulator Architectures

* Could try using the Feynmann path integral formulation of quantum mechanics
* Could use CUDA, Apple's Metal compute and standard Rust

## Installing on EC2

To run this on a p3.2xlarge instance, you will need to do the following:

```bash
sudo yum update -y
sudo yum install git -y
sudo yum groupinstall -y "Development tools"
sudo yum install -y kernel-devel-`uname -r`
wget https://developer.nvidia.com/compute/cuda/9.1/Prod/local_installers/cuda_9.1.85_387.26_linux
chmod +x cuda_9.1.85_387.26_linux

# MUST RUN MANUALLY ================
./cuda_9.1.85_387.26_linux
#===================================

sudo nvidia-smi -pm 1
sudo nvidia-smi -acp 0
sudo nvidia-smi --auto-boost-permission=0
sudo nvidia-smi -ac 2505,875

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

source $HOME/.cargo/env

git clone https://github.com/QCGPU/qcgpu-rust
cd qcgpu-rust
```
