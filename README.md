# TSconf 2019 - The Cost of Abstractions

As programmers we use abstractions all the time, but do we ever care about their costs? In this talk we’ll benchmark several implementations of a small but significant piece of code, using different abstractions to make it “better” but also measuring their cost in several dimensions.

In this talk we’ll focus on the cost of abstractions, and how they impact the quality of the resulting code.
We’ll consider the following costs:
- computational complexity (CPU resources used)
- code size (including eventual dependencies)
- eventually memory usage
- code complexity and maintainability (they should get better using abstractions)
- cognitive overhead (how hard it is to understand the abstraction)

The piece of code we’ll use is small enough to be understood by the audience in a minute, but it is realistic and could be used in production both in a front end and in a back end.

As abstractions we’ll gradually introduce types and functional constructs. Then we’ll also show a Rust implementation using the same abstractions (this is significant because the language claims to support “zero cost abstractions”), running both as native code and as a WASM module, and compare it to the Typescript one. Finally we’ll also investigate how AssemblyScript influences the costs.
