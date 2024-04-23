(module
  (func $functionToBeCalledFromWasm (import "imports" "functionToBeCalledFromWasm") (param i32))
  (func (export "functionToBeCalledFromJavaScript")
    i32.const 42
    call $functionToBeCalledFromWasm))
