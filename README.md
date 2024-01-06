# mock-radar

A server that mocks the QRadar server API behavior, to ease on developing integrations and avoid managing a full QRadar environment.

## Why not simply mock locally?

You can do this. You'll have to mimic on your own how the API behaves and duplicate code whenever is required.

But, I am trying to achieve an accurate replicate and do not wish to repeat myself as I can have many projects using the same mock.

I though I'd share my own work in case others find it useful. Who knows, maybe even collaborate and add their own mocked endpoints. This could be the start of something big. or not.

## Why write the mock server in Rust?

1. Small, low-profile, independent executable that doesn't require third party installations such as a Python, Java or .Net runtime. Can be compiled to any platform supported by LLVM (clang). You can use your old laptop.

2. Powerful type system with high code reliability, excellent for potentially big projects

3. Excellent standard toolchain for running, managing and testing projects

4. Growing community

## Collaboration

Everything here is licensed under MIT because I really do not see anything special here. It's simply a mock of the real thing.
If you wish to open an issues, share ideas, request features or send your own PR, feel free. We'll discuss about it as we go.
