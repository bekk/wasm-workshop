# Task 1

Run `npx serve` in this folder.

Look inside index.html and see how the WASM file is loaded.

Unfortunately the implementation is wrong. The answer should be 42.

Use `npx wasm2wat answer.wasm --output answer.wat` to generate a text representation of the file.

Make the necessary changes so that the function returns 42.

Convert it back to binary with `npx wat2wasm answer.wat --output answer.wasm`.

Refresh your browser and verify that the result is 42.