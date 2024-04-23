# Task 1 - Modify WASM

Look inside index.html and see how the WASM file is loaded.

Unfortunately the implementation is wrong. The answer should be 42.

Generate a text representation of the file:
```sh
npx wasm2wat answer.wasm -o answer.wat
```

Make the necessary changes so that the function returns 42.

Convert it back to binary:
```sh
npx wat2wasm answer.wat
```

Refresh your browser and verify that the result is 42.