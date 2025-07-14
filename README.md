# Mock-Radar

A lightweight, high-performance mock server that replicates QRadar API behavior for developing and testing QRadar-based applications without the overhead of managing an actual QRadar environment.

## Why Mock-Radar?

### The Problem with Custom Mocks

While you can write your own mocks for testing, this approach has limitations:
- Duplicated effort across projects
- Inconsistent API behavior simulation
- Time-consuming setup for each new project
- Lack of community-driven accuracy improvements

### Our Solution

Mock-Radar aims to provide the most accurate QRadar API replica through community collaboration. Instead of reinventing the wheel for every project, we offer a shared, continuously improved mock server that benefits everyone working with QRadar integrations.

## Key Advantages

### 🌱 **Resource Efficient**
- **No Virtual Machine Required**: Eliminate the need for heavy VM setups
- **Minimal System Requirements**: No 12GB RAM or 250GB storage overhead
- **Battery Friendly**: Significantly lower resource consumption
- **Cross-Platform**: Runs anywhere LLVM (clang) is supported

### ⚡ **Built with Rust**
- **Zero Runtime Dependencies**: Self-contained executable requiring no Python, Java, or .NET installations
- **Reliable Type System**: Robust error handling and type safety for large-scale projects
- **Excellent Toolchain**: Built-in testing, dependency management, and cross-compilation
- **Growing Ecosystem**: Active and supportive Rust community

## Project Goals & Roadmap

### Priority Endpoints

To maximize value, we prioritize endpoints based on:

1. **State-Mutating Operations**: Endpoints that update and retrieve data dynamically
2. **Most Common Use Cases**:
   - **Reference Sets** ✅ *Currently Implemented*
   - **Offenses** 🚧 *Next Priority*
   - **Log Sources** 🚧 *Planned*
   - **Custom Properties** 🚧 *Planned*
   - **Rules** 🚧 *Planned*
   - **Ariel Search** 🚧 *Future*

> **📋 [View Detailed Roadmap](ROADMAP.md)** - See development phases, ROI analysis, and timeline

> **Help Us Prioritize**: Open an issue or start a discussion to suggest which endpoints you need most. Pull requests with comprehensive tests are always welcome!

### AQL Engine Considerations

**Current Stance**: We do not plan to replicate QRadar's AQL engine due to:
- Complexity beyond project scope
- Potential copyright and patent concerns with IBM

**Future Possibility**: A configurable AQL response system where users can pre-define query-response mappings for testing scenarios may be considered for enhanced pipeline integration.

## Getting Started

### Installation
```bash
# Build from source (requires Rust)
git clone https://github.com/DK26/mock-radar
cd mock-radar
cargo build --release
```

### Basic Usage
```bash
# Start server on default port (3000)
cargo run

# Or run the built binary
./target/release/mock-radar
```

### API Examples
```bash
# List reference sets
curl -H "SEC: <your-token>" http://localhost:3000/api/reference_data/sets

# Create a reference set
curl -X POST \
  -H "SEC: <your-token>" \
  "http://localhost:3000/api/reference_data/sets?name=test_ips&element_type=IP"
```

## Community & Contribution

### Everyone Can Contribute
You don't need to be a developer! Contributions include:
- 📝 Documentation improvements and typo fixes
- 💡 Feature suggestions and use case discussions
- 🐛 Bug reports and testing feedback
- 🔧 Code contributions and API endpoint implementations

### Discussion & Support
Join our community discussions for:
- Feature requests and prioritization
- Implementation questions
- Integration experiences
- General QRadar development topics

**[💬 Start a Discussion](https://github.com/DK26/mock-radar/discussions)**

### Development Contributions
- All contributions are licensed under MIT License
- **Test-Driven Development**: We implement tests that mimic QRadar API behavior first, then adjust code until tests pass
- Follow existing code patterns and documentation standards

## Legal & Licensing

### MIT License
This project is licensed under the MIT License, promoting community collaboration and widespread adoption.

### Contribution Agreement
By contributing to this project, you agree that your contributions will be licensed under the MIT License.

### Disclaimer
**Important**: This project is an independent, unofficial community effort and has no affiliation with IBM Corporation or the QRadar product. IBM and QRadar are trademarks of IBM Corporation.

- Use at your own risk under MIT License terms
- [IBM Trademark Information](https://www.ibm.com/docs/en/zsms1/1.8.0?topic=notices-trademarks)

---

**Ready to mock your QRadar integration?** Get started with the installation guide above or join our community discussions!
