(module
  (import "" "displayResult" (func $displayResult (param i32)))
  (func $add (param $a i32) (param $b i32) (result i32)
    local.get $a
    local.get $b
    i32.add
  )
  (func $callerFunction
    (i32.const 5)
    (i32.const 7)
    (call $add)
    ;; pass the result which is at the top of the stack to the import display func
    (call $displayResult)
  )
  (export "callerFunction" (func $callerFunction))
)
