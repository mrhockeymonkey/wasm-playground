(module
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  (func $callerFunction
    ;; Prepare arguments
    (i32.const 5)   ;; Push the value 5 onto the stack as the first argument
    (i32.const 7)   ;; Push the value 7 onto the stack as the second argument

    ;; Call $add, the result will be put on the stack
    (call $add)

    ;; Consume the result by popping it from the stack
    drop
  )

  (export "callerFunction" (func $callerFunction))
)
