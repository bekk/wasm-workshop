# Webassembly

- Repo: https://github.com/bekk/wasm-workshop/
- Slides: https://bekk.github.io/wasm-workshop/

---

# Part 1

???

Ask the audience about their experience with WebAssembly

---

# Introduction

- Released in march 2017
- Goal was to enable high perforamnce applications on web pages
- Something something compile once run everywhere -> super duper portable
- WebAssembly 2.0 from 2022

---

# What is WASM?

- 001010101010101010101010101010101011101101010110
- Stack machine / VM / sandbox / low level place
- Statically typed, compiled AOT
- Only knows numerical types: i32, i64, f32, f64
- Can interop with JS
    - Call WASM functions from JS
    - Call JS functions from WASM
- Cannot access the DOM directly
- Shared memory

---

# Why use WASM?

- Make us of existing code
- Predictable (and fast) performance
- Low level control
- Portability
- You look cool as a developer

---

# How do I create WASM?

C source code:
```C
int factorial(int n) {
  if (n == 0)
    return 1;
  else
    return n * factorial(n-1);
}
```

WebAssembly .wat text format:
```
(func (param i64) (result i64)
  local.get 0
  i64.eqz
  if (result i64)
      i64.const 1
  else
      local.get 0
      local.get 0
      i64.const 1
      i64.sub
      call 0
      i64.mul
  end)
```

WebAssembly .wasm binary format:

```
00 61 73 6D 01 00 00 00
01 06 01 60 01 7E 01 7E
03 02 01 00
0A 17 01
15 00
20 00
50
04 7E
42 01
05
20 00
20 00
42 01
7D
10 00
7E
0B
0B
```

---

# But how do I load it???

```js
WebAssembly.instantiateStreaming(fetch("./my-wasm.wasm"))
    .then((module) => {
        // Do something with the module
    });
```

---

# Wat now?

Tasks 1 to 5.
Recap in a bit...

---

# Part 2

---

# Shared memory

- Data in/out
- Complex data structures
- Garbage collection?
- Difficult...

---

# Assemblyscript: Why

- WASM is tedious
- The interop is limited
- Simplify WASM calling JS
- Simplify JS calling WASM

---

# Assemblyscript: How

- WASM with a "Typescript-like" language
- Limited set of Typescript syntax
- ..with WASM types

---

# What now?

Task 6

---

# TODO

- Event loop
- Multi-threading

---

# WASM in the future

- Roadmap
- WASI
- WASM Components
- WASM GC

---

# Task 7

- Snake (Rust/Kotlin/AssemblyScript)
- [Yew](https://yew.rs/docs/tutorial) (Rust): [Tutorial](https://yew.rs/docs/tutorial)
- Blazor (C#)

#### Blazor

Run C# code in browser! 🤯

#### Yew

"React" written in Rust, compiled to WASM.

#### Snake  🍎 🐍

- Rust
- Kotlin
- AssemblyScript

TODO: Add boilerplate for all languages

---

# Resources

- https://yew.rs
- https://dioxuslabs.com/learn/0.5/
- https://github.com/leptos-rs/leptos/
- https://blog.ttulka.com/learning-webassembly-1-hello-world-of-wasm/
- https://blog.ttulka.com/learning-webassembly-2-wasm-binary-format/
- https://blog.ttulka.com/learning-webassembly-3-wat-programming-basics/
- https://blog.ttulka.com/learning-webassembly-4-wasm-memory-and-working-with-strings/
- https://blog.ttulka.com/learning-webassembly-5-running-wasm-in-the-browser/
- https://blog.ttulka.com/learning-webassembly-6-running-wasm-in-nodejs/
- https://blog.ttulka.com/learning-webassembly-7-introducing-wasi/
- https://blog.ttulka.com/learning-webassembly-8-compiling-into-wasm/
- https://blog.ttulka.com/learning-webassembly-9-assemblyscript-basics/
- https://blog.ttulka.com/learning-webassembly-10-image-processing-in-assemblyscript/
- https://wasmbyexample.dev/home.en-us.html
- https://www.youtube.com/watch?v=u0Jgz6QVJqg
- https://youtu.be/zqfF7Ssa2QI?si=FlkfOFEpH1ulMW6j
- https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format
- https://developer.mozilla.org/en-US/docs/WebAssembly/C_to_Wasm
- https://www.udemy.com/course/rust-webassembly-with-js-ts-the-practical-guide/?couponCode=2021PM20
- https://news.ycombinator.com/item?id=39960537
- https://github.com/mbasso/awesome-wasm
- https://github.com/ffmpegwasm/ffmpeg.wasm
