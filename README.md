# mock-radar

A server that mocks the QRadar server API behavior, to ease on developing integrations and avoid managing a full QRadar environment.

## Why write the mock server in Rust?

1. Small, low-profile, independent executable that doesn't require third party installations such as a Python, Java or .Net runtime. Can be compiled to any platform supported by LLVM (clang). You can use your old laptop.

2. Powerful type system with high code reliability, excellent for potentially big projects

3. Excellent standard toolchain for running, managing and testing projects

4. Growing community
