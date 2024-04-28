name: inverse
class: center, middle, inverse
---

template: inverse
# Webassembly

---

## Agenda

- 17:00 Mat
- 17:10 Slides part 1
- 17:30 Oppgave 1-5: samarbeid, snakk sammen, spør om hjelp
- 18:15 Recap oppgaver part 1
- 18:30 Slides part 2: Assemblyscript & Rust
- 19:00 Oppgave 6 og 7
- 20:00 Ferdig! De som vil forsetter.

???

17:00 Mat skal være på plass, spising
17:10 Slides part 1: Begynner presentasjon mens folk spiser
17:30 Oppgave 1-5, samarbeid, snakk sammen, spør om hjelp
18:15 Recap oppgaver part 1
18:30 Slides part 2: assemblyscript & rust
19:00 Oppgave 6, 7. Eller jew, eller blazor
20:00 Ferdig! De som vil forsetter.

---

template: inverse
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
- TODO: Event loop
- TODO: Multi-threading

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

template: inverse
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
- `npx asinit .`

---

# Assemblyscript: How

- WASM with a _Typescript-like_ language
- Limited set of Typescript syntax
- Initially only with WASM types `i32` & `f32`
- [Host bindings](https://www.assemblyscript.org/compiler.html#host-bindings) for [`esm`](https://tc39.es/ecma262/#sec-modules)
  - Number
  - String
  - Array
  - BigInt
  - ArrayBuffer
  - StaticArray
  - Object without constructors (by-copy)
  - Object with constructors (by-reference)

---

# Assemblyscript: JS -> WASM

Om man bygger med `--bindings esm`

```assemblyscript
// assembly/index.ts
export function doit(what: string): string {
  return `Did ${what}.`
}
```

```esm
// js
import { doit } from "./build/release.js";

console.log(doit("it")); // Did it.
```

---

# Assemblyscript: WASM -> JS

Om man bygger med `--bindings esm`

```assemblyscript
// assembly/index.ts
@external("env", "greet")
declare function greet(name: string): string;

export function run(): void {
  greet("torgeir")
}
```

```js
// js
import { run } from "./build/release.js";
globalThis.greet = (name) => console.log("Hello", name)
run() // Hello, torgeir
```
---

## Eller

Om man bygger med `--bindings raw`

```assemblyscript
// assembly/index.ts
@external("index", "greeter")
declare function greet(name: string): string;

export function run(): string {
  return greet("torgeir")
}
```

```js
// js
import { instantiate } from "./build/release.js";
const wasm = await instantiate(
  await WebAssembly.compileStreaming(await fetch("./build/release.wasm")),
  {
    index: {
      greeter: function (name) {
        console.log("Hello", name)
      },
    },
  },
);
wasm.run() // Hello, torgeir
```

---

# Assemblyscript: Objekter
_by-copy_

```assemblyscript
// assembly/index.ts
// `type` is not supported 😞
class Person { name!: string }
class Greeting { greeting!: string }

@external("env", "greeter")
declare function greet(_: Person): Greeting;

export function run(person: Person): Greeting {
  return greet(person)
}
```

```js
import { run } from "./build/release.js";
globalThis.greeter = ({ name }) => ({
  greeting: `Hello, ${name}`,
});

const person = { name: "torgeir" }
run(person).greeting; // Hello, torgeir
```

---

# Assemblyscript: Objekter
_by-reference_

```assemblyscript 
// assembly/index.ts
class Point {
  constructor(
    readobly x: i32,
    readobly y: i32
  )
}
export function newPoint(x: i32, y: i32): Point {
  return new Point(x, y);
}
export function pointToString(p: Point): string {
  return `Point(${p.x}, ${p.y})`;
}
```

```js
// js
import { newPoint, pointToString } from "./build/release.js";

let point = newPoint(2, 4);
console.log(point); // Number { 37920 }

pointToString(point); // Point(2, 4)
```

---

---

# Rust: Why Rust?

- System programming language
  - Fast and memory-efficient
  - No runtime or garbage collector
- Features:
  - Memory safety, thread safety
  - Rich type system
  - Modern tooling
  - High-level, zero-cost abstractions

---

# Rust and Webassembly

- The advantages of Rust, in the browser or Node
- Good interop with JS, ES modules
- Small code size
- Rust/Wasm book: https://rustwasm.github.io/docs/book/

---

# Rust / JS interop

- Call Rust functions from JS
  - Or JS functions from Rust
- Generate glue code automatically
  - ✅ Send/receive strings
  - ✅ Send/receive complex data types (lists, objects)

---

# What now?

- Task 6: AssemblyScript
- Task 7: Snake game with Rust and Webassembly
  - Optionally AssemblyScript or Kotlin
- Or, do what ever you want. Maybe check out:

#### Blazor

[Run C# code in browser!](https://dotnet.microsoft.com/en-us/learn/aspnet/blazor-tutorial/intro) 🤯

#### Yew

["React" written in Rust, compiled to WASM](https://yew.rs/docs/tutorial)

#### Kotlin

Compile Kotlin to WASM, or [Kotlin Compose Multiplatform](https://kotlinlang.org/docs/wasm-get-started.html)

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
