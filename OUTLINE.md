# Outline

## Introduction

* What is Rust?
* Why use Rust?
* Why not use Rust?

## 1. Primitives

* strings
* numbers
* casting
* booleans
* mutability

## 2. Collections

* tuples
* structs
* enums
* pattern matching

## 3. Arrays

* arrays
* methods
* Vec

## 4. Ownership

* Stack memory
* Heap memory
* Ownership
* Moving

## 5. Borrowing and Lifetimes

* Borrowing [get into scopes - talk about blocks and how reassignment with `let` triggers dropping]
* Slices [array *and* string]
* Lifetimes
* Static lifetimes

## 6. Traits
* Display and Debug
* From and Into [introduce the Self type here?]
* Clone and Copy [introduce the Self type here?]
* Iterators [into_iter vs iter vs iter_mut]

## 7. Closures
* Option and Result
* First-class functions
* Closures

## 8. Modules, Crates, and Cargo
* modules
* crates [turbofish, e.g. `std::mem::size_of::`]
* cargo
* tools [clippy, fmt, bench, unused deps...]
