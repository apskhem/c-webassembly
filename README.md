<p align="center">
  <img width="100" src="c-webassembly.png" alt="c-webassembly-logo">
</p>

<div align="center">
  
## C-WebAssembly
[Demo](/docs/#readme) — [Documentation](/docs/#readme)
  
  <a href="https://github.com/rust-secure-code/safety-dance/"><img src="https://img.shields.io/badge/version-0.1.0--alpha.1-critical?style=flat-square" alt="badge" height="20" /></a>
  <a href="https://github.com/rust-secure-code/safety-dance/"><img src="https://img.shields.io/badge/status-developing-8da0cb?style=flat-square" alt="badge" height="20" /></a>
  <a href="https://github.com/apskhem/c-webassembly/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-Apache_2.0-blue?style=flat-square" alt="badge" height="20" /></a>
  <a href="https://github.com/rust-secure-code/safety-dance/"><img src="https://img.shields.io/badge/unsafe-forbidden-success?style=flat-square" alt="badge" height="20" /></a>

</div>

**C-WebAssembly** is a programming language designed specifically for writing WebAssembly in C-like syntax as close to the machine as possible, with manual table and memory management and native instructions. The language will be compiled into .wat or .wasm, without any boilerplate or over-generated code (except built-in libraries). With C-WebAssembly, it is possible to manage memory and table manually. The compiler aims to provide a zero-cost abstraction feature that guarantees compiled code is fully optimized.

### Sample Syntax

[view simple code sample](tests/samples/simple.cwal)