# Project Introduction
  This is the source code of an unaccepted paper.
# prerequisite
1. Python environment
   Python>=3.10
2. Rust environment
   1. Rust is best for the Nightly version。
   Switching versions:
   ```bash
   rustup default nightly
   ```
   2. Install LLVM tool:
   ```bash
    rustup component add llvm-tools-preview
   ```
3. GCC environment
   gcc = 10.xx
4. Cmake environment
5. Other packages
   ```bash
   sudo apt install libboost-all-dev
   sudo apt install jq
   cargo install cargo-bolero
   ```
# Install
1. Clone the project locally:
   ```bash
   git clone https://github.com/RX-Zhang/RustFlow.git
   ```
2. Install dependencies
   ```bash
   pip install -r requirements.txt
   ```
4. run program（Need network）
   ```bash
   python driver.py --benchmark-name arg1 --submodule-name arg2
   ```
  
