# Task 4 - Write to shared memory from JavaScript

Shared memory can also be created and exported from WASM.

In this task you will write a value to the shared memory from JavaScript.

When you call `read` with an offset, the value is read and returned from WASM.

Unfortunately it only works 10% of the time.

Build WASM:
```sh
npx wat2wasm wasm-memory.wat
```
