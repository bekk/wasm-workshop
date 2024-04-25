(module
  (memory $memory 1)

  (func $read (param $offset i32) (result i32)
    (i32.load (local.get $offset))
  )

  (export "memory" (memory $memory))
  (export "read" (func $read))
)