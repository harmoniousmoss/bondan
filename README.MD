# 🧠 rust-compile-stress

A simple Rust project that generates 1,000 unique structs using macros to **stress test Rust compile time performance**.  

Created for benchmarking how different systems (e.g. MacBook Pro vs Mac Mini) handle large-scale Rust macro expansions and code generation workloads.

---

## 🚀 Purpose

This project is designed to:
- Benchmark **Rust compile time** under high code generation load
- Help compare performance between devices with the same CPU (e.g., Apple Silicon)
- Showcase Rust’s macro system and compile-time expansion behavior

---

## 📦 What It Does

- Generates `Struct1` to `Struct1000` using a custom macro
- Each struct has:
  - A `new()` constructor
  - A `compute()` method that returns the square of the struct's number
- Prints `Struct1000::compute()` output during runtime
