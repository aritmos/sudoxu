# 🧩 Sudoxu

`Sudoxu` is a more *human-like* sudoku solver.

- It aims to be much faster than standard brute force backtracking algorithms.
- It aims to solve sudokus with the same techniques that humans do.


The core implementation of `sudoxu` however does differ from human computations of a sudoku. `Sudoxu` works by systematically reducing the phase space of the board until it has been solved. It begins with all cells containing all pencilmarks and simply removes pencilmarks using the standard human techniques. When a cell only contains one remaining pencilmark its value its known.


The `sudoxu` codebase (will eventually) consist of three main modules:

1. The core sudoku solving functionality:
    - Input and output of grids and current states
    - Methods to remove candidates (pencilmarks) from the grid
2. Solvers:
    - Structures that implement the `Solver` trait. These manage all of the available methods to solve the sudoku. They are the "brains" that set the solving strategy.
3. UI:
    - A TUI interface.
    - Ability to select different solvers, including step by step solutions.
    - Metrics and other pretty outputs.

## 🚧 Project Status

- This project is still in early works. It will exist as a library until all major implementations have been completed. After that it wil be turned into a binary with a terminal interface. The project will be hosted in [crates.io](https://crates.io/) when it is closer to `v1.0`. For now the project can be explored via its documentation:
    - For now accessible on my [github.io page](https://aritmos.github.io)
    - Always available by cloning the repo and running: `cargo doc --open`.

---

- Currently the project is being rewritten in the [rewrite branch](https://github.com/aritmos/sudoxu/tree/rewrite), as I polish the original code that I wrote for the project over the summer.

- To view the current implementation state and the project roadmap, visit the issues and projects tabs.

- The development of the more advanced solving strategies is currently on hold until I get a hold of a new CPU that supports the AVX512 instruction set along with specific extensions. This is essential for implementing the more intricate algorithms involving efficient bit manipulation that I have come up with.

## 🔭 Project Focus
Sudoxu is the successor to my original code-golf backtracking [Python implementation](https://gist.github.com/aritmos/abd51b581c261ce6ce9c25511e5ea7e7) aimed to make use of the lower-level capabilities of Rust, and pushing my knowledge of algorithms and memory manipulation. In this project I am heavily focusing on:

- **Memory and Dependencies**: A heavy emphasis on the memory footprint of the program and standard library usage over external libraries. The final implementation could very well be able to run on embedded systems. 
- **Algorithms, Assembly and Vectorisation**: A close monitoring of the compiled implementations using [Compiler Explorer](https://rust.godbolt.org/) and [cargo-show-asm](https://crates.io/crates/cargo-show-asm) to optimise functions. I am already using some SIMD intrinsics with the use of the [Intel Intrinsics Guide](https://www.intel.com/content/www/us/en/docs/intrinsics-guide/index.html) thanks to Rust's [vendor intrinsics support](https://doc.rust-lang.org/core/arch/index.html). As this is a personal project, I will be working with what's available to my machine: intel (`x86_64`) cpu supporting `SSE4.1`, `SSE4.2` and `AVX2` instruction sets.
- **Benchmarking**: Current focus on microbenchmarks using Rusts in-built benchmarking, [hyperfine](https://crates.io/crates/hyperfine), [easybench](https://crates.io/crates/easybench) and [criterion](https://crates.io/crates/criterion). Expecting to improve the procedure by benchmarking longer implementation chains once I have finished writing the core logic.
