# Task 3 - Write to shared memory from WASM

Functions imported or exported with WASM can only be called with, or return, integers and floats.
This means that any other data, like strings, must be binary encoded and accessed through shared memory.

This task runs a WASM file that imports a page of shared memory and writes a greeting to it.

Look inside the WAT in order to determine the offset in the shared memory and the length of the greeting in order to display it.

Build WASM:
```sh
npx wat2wasm memory.wat
```
