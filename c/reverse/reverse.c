#include "duktape.h"

#define WASM_IMPORT(m,n) __attribute__((import_module(m))) __attribute__((import_name(n)));
#define WASM_EXPORT(n) asm(n) __attribute__((visibility("default")))

int dfn_ads(void) WASM_IMPORT("ic0", "msg_arg_data_size");
void dfn_adc(void *, int, int) WASM_IMPORT("ic0", "msg_arg_data_copy");
void dfn_reply_append(void *, int) WASM_IMPORT("ic0", "msg_reply_data_append");
void dfn_reply(void) WASM_IMPORT("ic0", "msg_reply");
void dfn_print(void *, int) WASM_IMPORT("ic0", "debug_print");

void go() WASM_EXPORT("canister_update go");
void go() {
  char buf[128];
  int sz = dfn_ads();
  dfn_adc(buf, 0, sz);

  // Encoded string: "DIDL" 0 1 0x71 LEB128(length) data
  // So offset 7 holds string length (for short strings).
  int n = buf[7];
  
  duk_context *ctx = duk_create_heap_default();
  duk_eval_string(ctx, "'a string'");
  const char *str;

  str = duk_get_string(ctx, -3);
  
  dfn_print(str);
  dfn_reply_append(str, 8);
  dfn_reply();
}
