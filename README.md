# mock-radar

A highly minimalist mock server that replicates QRadar API behavior, as an aid for developing and integrating QRadar based applications by avoiding the entire setup and management of an actual QRadar environment.

## Why not simply write my own mock for testing?

You can do this. You'll have to mimic on your own the API behavior and duplicate code whenever is required. Nothing wrong with that.

However, what I am trying to achieve with this project is having the most accurate replica, which would require a community help, and do not wish to rewrite it over and over again as I may have multiple projects requiring the same mock logic.

I though I'd share my own work in case others find it useful. Who knows, maybe even collaborate and add their own mocked endpoints. This could be the start of something.

### Other Advantages

- __Green__: Using less resources and saving on battery life, as you do not need to run a heavy virtual machine in order to develop for QRadar API, which would otherwise require:
  - Installation & Configurations of a virtual machine environment

  - 12 GB of RAM (minimum)
  - 250 GB of available storage size (minimum)

## Why write the mock server in Rust?

1. Small, low-profile, independent executable that doesn't require third party dependencies or environments, such as Python, Java(JDK) or .Net runtime installations. It can simply be compiled to any platform supported by LLVM (clang). You can even run it on your old laptop (a.k.a green technology).

2. Very powerful and reliable type system for coding correctly which is also excellent for potentially big projects

3. Excellent standard toolchain for running, managing and testing projects

4. Having a growing community

## Project Goals & Priorities

### Endpoints to Replicate

In order to produce the most value out of this tool, we need to set proper priority categories for endpoint replica: 

  - Endpoints that perform state mutations (e.g, Update and retrieval of data to and from QRadar, dynamically)
  - Most commonly used endpoints:
    - ReferenceSets
    - Offenses
    - CustomActions
    - LogSources
    - Properties
    - Rules

  Feel free to help us prioritize by opening a ticket or a discussion. Pull requests with working tests are welcomed.

### AQL Engine Replication is a Non-Priority, But!

We do not seek to replicate the AQL engine, because it would be both too complex and beyond the scope of this project. It may also, potentially, violate copyrights and patents by IBM. 

However, a fake AQL mechanism where the user can predefine and match a pre-configured result for a pre-configured AQL query, may be available in the future, to allow for a more holistic experience when using this tool for testing in your pipeline

###



## Contribution, Collaboration & Licensing

### Community

You don't have to be a software developer! Feel free to [share your input in the discussions section](https://github.com/DK26/mock-radar/discussions/) and talk about anything.

Contribution could be anything from typo fixes, to sharing ideas, suggestions, issues, experiences, and PRs

### MIT License

Everything here is licensed under MIT license since it's simply a mock that mimics API, and shouldn't be something more than a community effort

### Contribution Licensing

You agree that any contribution is licensed under the MIT license

### Disclaimer

This project has nothing to do with the IBM(R) company or the QRadar (IBM's trademark) project and is simply an unofficial community effort. Use at your own risk under the restrictions of the MIT license

- IBM Trademarks: [https://www.ibm.com/docs/en/zsms1/1.8.0?topic=notices-trademarks](https://www.ibm.com/docs/en/zsms1/1.8.0?topic=notices-trademarks)  
