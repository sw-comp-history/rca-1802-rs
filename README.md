# RCA COSMAC 1802 Assembly Emulator

An interactive browser-based educational game for learning the RCA COSMAC 1802 instruction set architecture. The RCA 1802 was the first CMOS microprocessor, introduced in 1976 and famous for its use in NASA space missions including Galileo, Hubble Space Telescope, and Voyager.

## Live Demo

**[Try it online](https://sw-comp-history.github.io/rca-1802-rs/)**

![RCA 1802 Emulator Screenshot](images/screenshot.png?ts=1772044849000)

## Features

- **Historical RCA 1802 CPU emulation** - 64KB memory, 16 general-purpose registers
- **Unique register architecture** - P (program counter selector), X (index selector)
- **Complete instruction set** - LDI, PLO, PHI, GLO, GHI, STR, LDN, ADD, SUB, AND, OR, XOR, SHL, SHR, branches
- **Interactive examples** covering register loading, arithmetic, loops, and conditionals
- **Progressive challenges** with validation
- **Assembly parser** with label support
- **Real-time visualization** of CPU state, registers, and memory

## Documentation

- [Architecture](docs/architecture.md) - System design and component structure
- [Porting Guide](docs/porting.md) - How to port assembly games to public repos

## Architecture

The RCA 1802 is a unique 8-bit processor with:

- **R0-RF** - 16 general-purpose 16-bit registers (any can be PC or index)
- **D** - 8-bit accumulator for arithmetic
- **P** - 4-bit program counter selector (which register is PC)
- **X** - 4-bit index register selector
- **DF** - Data flag (carry/borrow)
- **Q** - Single-bit output
- **IE** - Interrupt enable

## Space Heritage

The RCA 1802 was chosen for numerous NASA missions due to its radiation-hardened CMOS design:

- **Galileo** - Jupiter orbiter (1989-2003)
- **Magellan** - Venus radar mapper (1989-1994)
- **Hubble Space Telescope** - Wide Field Planetary Camera
- **Ulysses** - Solar polar orbiter (1990-2009)
- **Voyager 1 & 2** - Interstellar probes (launched 1977)

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (with wasm32-unknown-unknown target)
- [Trunk](https://trunkrs.dev/) - `cargo install trunk`

### Development

```bash
# Run development server with hot reload
trunk serve

# Build for production
trunk build --release
```

The production build outputs to `./pages/`.

### Deploying to GitHub Pages

1. Build locally:
   ```bash
   trunk build --release
   ```

2. Update gh-pages branch:
   ```bash
   git checkout gh-pages
   rm -rf *.js *.wasm *.css index.html
   cp -r pages/* .
   git add .
   git commit -m "Deploy"
   git push
   git checkout main
   ```

## Project Structure

```
rca-1802-rs/
├── src/                    # Main application
│   ├── app.rs             # Yew application component
│   ├── assembler.rs       # Assembly parser
│   ├── cpu/               # CPU emulation
│   │   ├── executor.rs    # Instruction execution
│   │   ├── instruction.rs # Instruction definitions
│   │   └── state.rs       # CPU state management
│   ├── lib.rs             # Library root
│   └── wasm.rs            # WASM bindings
├── components/            # Shared Yew UI components
│   └── src/
│       ├── components/    # UI components (header, sidebar, etc.)
│       └── lib.rs
├── styles/                # CSS stylesheets
├── docs/                  # Documentation
├── images/                # Screenshots
├── index.html             # HTML entry point
├── Trunk.toml             # Trunk configuration
└── Cargo.toml             # Workspace configuration
```

## References

### RCA Documentation

- **[CDP1802 User Manual (MPM-201A)](http://www.bitsavers.org/components/rca/cosmac/MPM-201A_CDP1802_Users_Manual_Nov77.pdf)** - Official RCA COSMAC 1802 user manual (1977)
- **[COSMAC Microprocessor](https://en.wikipedia.org/wiki/RCA_1802)** - Wikipedia overview

### Historical Context

- Joseph Weisbecker designed the 1802 at RCA in 1974-1976
- First CMOS microprocessor - extremely low power consumption
- Radiation-hardened variants (CDP1802A) used in space
- Featured in Popular Electronics as the "COSMAC ELF" computer kit

## License

MIT
