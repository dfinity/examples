#include <string.h>
#include "qrcodegen.h"

size_t strlen(const char* s) {
  int n = 0;
  while (s[n++]);
  return n;
}
void *memset(void *s, int c, size_t n) {
  char *p = s;
  while (n--) *p++ = c;
  return s;
}
void *memchr(const void *s, int c, size_t n) {
  const char *p = s;
  while(n--) if (*p++ == c) return (void *) p;
  return 0;
}
void* memmove(void *dst, const void *src, size_t n) {
  char *d = dst;
  const char *s = src;
  if (d <= s) {
    while (n--) *d++ = *s++;
  } else {
    s += n;
    d += n;
    while (n--) *d-- = *s--;
  }
  return dst;
}

char msg[2048] = "DIDL\0\x01\x71", *p;
void append(const char* s) { while (*s) *p++ = *s++; }

#define WASM_IMPORT(m,n) __attribute__((import_module(m))) __attribute__((import_name(n)));
#define WASM_EXPORT(n) asm(n) __attribute__((visibility("default")))

void reply_append(void*, int) WASM_IMPORT("ic0", "msg_reply_data_append");
void reply(void) WASM_IMPORT("ic0", "msg_reply");

// Adapted from basic demo in https://github.com/nayuki/QR-Code-generator.
void printQr(const uint8_t qrcode[]) {
	int size = qrcodegen_getSize(qrcode);
	int border = 4;
	for (int y = -border; y < size + border; y++) {
		for (int x = -border; x < size + border; x++) {
			append(qrcodegen_getModule(qrcode, x, y) ? "##" : "  ");
		}
		append("\n");
	}
	append("\n");
}

void basic(void) {
	const char *text = "Hello, world!";                // User-supplied text
	enum qrcodegen_Ecc errCorLvl = qrcodegen_Ecc_LOW;  // Error correction level
	
	// Make and print the QR Code symbol
	uint8_t qrcode[qrcodegen_BUFFER_LEN_MAX];
	uint8_t tempBuffer[qrcodegen_BUFFER_LEN_MAX];
	bool ok = qrcodegen_encodeText(text, tempBuffer, qrcode, errCorLvl,
		qrcodegen_VERSION_MIN, qrcodegen_VERSION_MAX, qrcodegen_Mask_AUTO, true);
	if (ok)
		printQr(qrcode);
}

void go() WASM_EXPORT("canister_update go");
void go() {
  // *@msg = 68; *@(msg + 1) = 73; *@(msg + 2) = 68; *@(msg + 3) = 76; *@(msg+4) = 0; *@(msg + 5) = 1; *@(msg + 6) = 113;
  p = msg + 12;
  basic();
  int n = p - msg - 12;

  msg[7] = (n & 255) | 128;
  msg[8] = ((n >> 7) & 255) | 128;
  msg[9] = ((n >> 14) & 255) | 128;
  msg[10] = ((n >> 21) & 255) | 128;
  msg[11] = n >> 28;

  reply_append(msg, p - msg);
  reply();
}
