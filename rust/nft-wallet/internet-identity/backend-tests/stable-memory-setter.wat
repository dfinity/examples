(module
  (import "ic0" "stable_write"
    (func $stable_write (param $offset i32) (param $src i32) (param $size i32)))
  (import "ic0" "stable_grow" (func $stable_grow (param i32) (result i32)))
  (import "ic0" "msg_arg_data_copy"
    (func $msg_arg_data_copy (param i32) (param i32) (param i32)))
  (import "ic0" "msg_arg_data_size"
    (func $msg_arg_data_size (result i32)))

  (func $canister_init
    (local $p i32)
    (set_local $p
      (i32.div_u
        (i32.add (call $msg_arg_data_size) (i32.const 65535))
        (i32.const 65536)))
    (drop (memory.grow (get_local $p)))
    (drop (call $stable_grow (get_local $p)))
    (call $msg_arg_data_copy (i32.const 0) (i32.const 0) (call $msg_arg_data_size))
    (call $stable_write (i32.const 0) (i32.const 0) (call $msg_arg_data_size))
  )
  (memory $memory 1)
  (export "memory" (memory $memory))
  (export "canister_init" (func $canister_init))
)
