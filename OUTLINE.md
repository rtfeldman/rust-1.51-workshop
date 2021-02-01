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

* structs [ intro to memory as a concept ]
* tuples
* enums
* methods (`impl`)

## 3. Parameterized Types

* arrays [ compare to structs - both are fixed-layout! ]
* Vec [ intro to dynamic-layout memory as a concept, contrast with structs and arrays ]
* Type variables
(maybe motivate Option and Result with String.head or something? Something that returns an Option or Result)

## 4. Ownership

* Stack memory
* Heap memory
* Ownership
* Scopes [talk about blocks and how reassignment with `let` triggers dropping]

## 5. Borrowing and slices

* Borrowing
* Array slices
* String slices

## 6. Lifetimes

* Lifetimes
* Lifetime Annotations
* Static lifetimes

## 7. Traits
* Display and Debug
* iterators
* From and Into
* AsRef

## 8. Modules, Crates, and Cargo
* modules
* crates
* cargo
* tools [clippy, fmt, bench, unused deps...]
