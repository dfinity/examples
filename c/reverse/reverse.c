// Reverse a string in place.
void reverse(char*s,int len) {
  char*t = s+len-1,c;
  while(s<t) {
    c = *t;
    *t = *s;
    *s = c;
    s++;
    t--;
  }
}

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
  reverse(buf+8, n);
  dfn_print(buf+8, n);
  dfn_reply_append(buf, sz);
  dfn_reply();
}
