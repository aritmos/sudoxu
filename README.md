# 🧩 Sudoxu

`Sudoxu` aims to be a *human-like* sudoku solver. In the sense that:

- It aims to be orders of magnitude faster than the standard brute force backtracking algorithms found in the web.
- It aims to solve sudokus with the same techniques that humans do. Within future implementations this means it could help a human find the next step if they are stuck solving a sudoku.

## 🔭 Project Focus
Coming from higher level languages such as Python and Julia I hope to take advantage of the lower level characteristics of Rust. Sudoxu is the successor to my original [code-golf backtracking implementation](https://gist.github.com/aritmos/abd51b581c261ce6ce9c25511e5ea7e7) aimed to be algorithmically smart and magnitudes faster. In order to accomplish this I am heavily focusing on:

- **Memory and Standard Library**: A heavy emphasis on the memory footprint of the program and standard library usage over external libraries. The final implementation could very well be able to run on embedded systems. 
- **Algorithms, Assembly and Vectorisation**: A close monitoring of the compiled implementations using [Compiler Explorer](https://rust.godbolt.org/) and [cargo-show-asm](https://crates.io/crates/cargo-show-asm) to optimise functions. I am already using some SIMD intrinsics with the use of the [Intel Intrinsics Guide](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html) thanks to Rust's [vendor intrinsics support](https://doc.rust-lang.org/core/arch/index.html). As this is a personal project, I will be working with what's available to my machine: `x86_64` architecture with an 8th Gen Core i7 supporting `SSE4.1`, `SSE4.2` and `AVX2` instruction sets.
- **Benchmarking**: Current focus on microbenchmarks using Rusts in-built benchmarking, [hyperfine](https://crates.io/crates/hyperfine), [easybench](https://crates.io/crates/easybench) and [criterion](https://crates.io/crates/criterion). Expecting to improve the procedure by benchmarking longer implementation chains once I have finished implementing the core logic.

## 🚧 Roadmap
- `v1.0` Core implementations
- `v2.0` Human guided solver
- `v3.0` Autonomous solver

## 📜 Project Status (towards `v1.0`)
```
💼 = Work in progress
🤔 = May or may not be implemented
```

### Core Structure
- [x] Board and cells
- [x] Sections of the board
- [x] Base implementations 
- [x] Ouput parsing

### Core Logic (Solving techniques)
Sudoku techniques are universal. I am mainly following [this site's](https://www.sudokuoftheday.com/techniques) naming conventions and difficulty ordering within my implementations.
- [x] Easy (Single position and single candidate)
- [x] Medium (Candidate lines, double lines, multiple lines)
- [ ] Advanced 💼 (Naked and hidden subsets)
- [ ] Master 🤔 (Nishio)  

### Unit Testing
- [ ] Input Parsing 💼
- [ ] Base implementations
- [ ] Finders
- [ ] Filters

### Benchmarking
- [ ] Bench competing implementations

## Possible Implementations
- [ ] SIMD 💼
- [ ] Multithreading 🤔

## Current Implementation

`TODO`

See in-file documentation

