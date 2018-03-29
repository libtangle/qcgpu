# How to install onto an EC2 p3.2xLarge instance

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

curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly

# MUST RUN MANUALLY ================
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly
#===================================


source $HOME/.cargo/env

git clone https://github.com/QCGPU/qcgpu-rust
cd qcgpu-rust
```
