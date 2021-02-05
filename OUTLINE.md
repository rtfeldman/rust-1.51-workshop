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

## 3. Arrays and Vectors

* arrays
* methods
* Vec

## 4. Ownership

* Stack memory
* Heap memory
* Ownership
* Moving

## 5. Borrowing and slices

* Borrowing
* Array slices
* String slices
* Scopes [talk about blocks and how reassignment with `let` triggers dropping]

## 6. Lifetimes

* Option and Result (maybe motivate Option and Result with String.head or something? Something that returns an Option or Result)
* Lifetimes
* Lifetime Annotations
* Static lifetimes

## 7. Traits
* Display and Debug
* From and Into [ introduce the Self type here]
* Clone and Copy
* iterators

## 8. Modules, Crates, and Cargo
* modules
* crates [turbofish, e.g. `std::mem::size_of::`]
* cargo
* tools [clippy, fmt, bench, unused deps...]
