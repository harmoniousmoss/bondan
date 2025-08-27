## 🧠 Bondan - Performance Benchmarking Tool

Bondan is a Rust project that generates 1,000 unique structs to test compile-time performance across different hardware configurations. Built to explore Rust's macro system and understand compile-time code generation.

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
