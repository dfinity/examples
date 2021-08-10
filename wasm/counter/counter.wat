(module
  ;; Import IC system API
  (import "ic0" "msg_reply" (func $msg_reply))                                 ;; msg_reply : () -> ()
  (import "ic0" "msg_reply_data_append" (func $reply_append (param i32 i32)))  ;; msg_reply_data_append : (src : i32, size : i32) -> ()
  (import "ic0" "msg_arg_data_size" (func $arg_size (result i32)))             ;; msg_arg_data_size : () -> i32
  (import "ic0" "msg_arg_data_copy" (func $arg_copy (param i32 i32 i32)))      ;; msg_arg_data_copy : (dst : i32, offset : i32, size : i32) -> ()
  (import "ic0" "trap" (func $trap (param i32 i32)))                           ;; trap : (src : i32, size : i32) -> ()

  ;; Export functions to IC
  (export "canister_query get" (func $get))
  (export "canister_update inc" (func $inc))
  (export "canister_update set" (func $set))

  ;; Single memory
  (memory 1)
  ;; Global counter
  (global $counter (mut i64) (i64.const 0))
  
  ;; Scratch space for int64 Candid value at offset 0
  ;; Layout:
  ;; +---------------------+---------------------+-------------------+------------------------+-------------------+
  ;; | magic number (DIDL) | type table size (0) | num of values (1) | 1st value type (int64) | little endian i64 |
  ;; +---------------------+---------------------+-------------------+------------------------+-------------------+
  (data (i32.const 0) "DIDL\00\01\74")
  ;; Empty candid message
  (data (i32.const 15) "DIDL\00\00")
  ;; Deserializer error message
  (data (i32.const 21) "Invalid input argument")

  ;; Define `get` function, which has to be of type () -> ()
  ;; At the Candid level, the type is () -> (int64)
  (func $get
    ;; Construct the return Candid message
    ;; Since the int64 value encoding in Candid is exactly the same as i64 in Wasm,
    ;; we can directly store the counter value at offset 7, which immediately follows the Candid header defined in data 0
    (i32.const 7)
    (global.get $counter)
    (i64.store)

    ;; return int64
    (i32.const 0)      ;; message src at offset 0
    (i32.const 15)     ;; size of the message: 7 + 64 / 8 = 15
    (call $reply_append)
    (call $msg_reply)
  )

  ;; Define `inc` function with Candid type () -> ()
  (func $inc
    ;; Increment the counter
    (global.get $counter)
    (i64.const 1)
    (i64.add)
    (global.set $counter)

    ;; return ()
    (i32.const 15)
    (i32.const 6)
    (call $reply_append)
    (call $msg_reply)
  )

  ;; Define `set` function with Candid type (int64) -> ()
  (func $set
    ;; Check message size
    (call $arg_size)
    (i32.const 15)
    (i32.eq)
    (call $assert_eq)

    ;; Store input message at offset 50 as scratch space for decoding
    (i32.const 50)  ;; dst
    (i32.const 0)   ;; offset
    (call $arg_size)
    (call $arg_copy)

    ;; Check message header, ignoring subtyping
    (i32.const 50)
    (i64.load)
    ;; Remove the last byte, as the expected Candid header is only 7 bytes
    (i64.const 8)
    (i64.shl)
    (i64.const 0x7401_004c_4449_4400) ;; expects "DIDL\00\01\74" in little endian
    (i64.eq)
    (call $assert_eq)

    ;; Load int64 value at offset 57
    (i32.const 57)
    (i64.load)
    ;; Store the value to counter
    (global.set $counter)

    ;; return ()
    (i32.const 15)
    (i32.const 6)
    (call $reply_append)
    (call $msg_reply)
  )
    
  (func $assert_eq (param $b i32)
    (local.get $b)
    (br_if 0)
    (i32.const 21)
    (i32.const 22)
    (call $trap)
  )
)
