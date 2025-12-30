## RSX

## Overview

1. What this project is about
2. The DOM layer
3. Fine-grained reactivity using Rust

## What this project is about.

Disclaimer: This project is not a new library nor a new frontend framework.

It is a study, an archive of my work. Most of this code is from sycamore-rs.

---

At first, I created this project to archive my work/training on Rust procedural macros. The base project was about implementing JSX (a famous syntax in modern frontend frameworks) in Rust, hence the name RSX.

After realizing that a "frontend syntax" had no adequate use in Rust.
I decided to associate it with WebAssembly for a "frontend-related" project.

I already had recreated React and liked deep-diving into this project but I didn't understand many things and had not documented my work like I did for this RSX project/archive.

This project is about recreating and understanding the core of reactive libraries using "fine-grained reactivity" like Sycamore and SolidJS.
For this project, I've "created" 4 libraries:

- rsx: A library that provides types for building UI elements such as components.

- rsx-macros: Rust macros that allow the creation of rsx elements using a JSX-like syntax.

- rsx-reactive: The main reactive library copied from Sycamore source code.

- rsx-web: a library that uses the rsx-reactive library to create reactive UI on the browser.

## The DOM layer

This project is a reactive library built in Rust. Rust will be used to produce WebAssembly that can interact with native JavaScript DOM elements:

```mermaid
flowchart TD;
    Rust-->|Compiles to|WebAssembly
    WebAssembly-->|Bundled/built using|Vite
    Vite-->|deliver Javascript|Browser
```
