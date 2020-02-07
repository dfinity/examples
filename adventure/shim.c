#define WASM_IMPORT(m,n) __attribute__((import_module(m))) __attribute__((import_name(n)));
#define WASM_EXPORT(n) asm(n) __attribute__((visibility("default")))

void print(void *, int) WASM_IMPORT("ic0", "debug_print");

int arg_size(void) WASM_IMPORT("ic0", "msg_arg_data_size");
void arg_copy(void *, int, int) WASM_IMPORT("ic0", "msg_arg_data_copy");
void reply_append(void*, int) WASM_IMPORT("ic0", "msg_reply_data_append");
void reply(void) WASM_IMPORT("ic0", "msg_reply");

void exit(int n) {
  print("exit", 4);
  if ((n - n)/(n - n)) for(;;); else for(;;);
}
extern unsigned __heap_base;
void* malloc(unsigned long n) {
  static unsigned bump = (unsigned) &__heap_base;
  return (void *) ((bump += n) - n);
}

typedef char *FILE;

void *calloc(unsigned long nmemb, unsigned long size) { return malloc(nmemb * size); }

void *memset(void *s, int c, unsigned long n) {
  char *p = s;
  while (n--) *p++ = c;
  return s;
}

unsigned long fread(void *ptr, unsigned long size, unsigned long nmemb, FILE *stream) {
  char *s = ptr;
  for (unsigned long n = size*nmemb; n; n--) *s++ = *(char *)(*stream)++;
  return nmemb;
}

unsigned long fwrite(const void *ptr, unsigned long size, unsigned long nmemb, FILE *stream) {
  const char *s = ptr;
  for (unsigned long n = size*nmemb; n; n--) *(char *)(*stream)++ = *s++;
  return nmemb;
}

int fclose(FILE *stream) { return 0; }

// Assumes 0 at end of input.
char *fgets(char *s, int size, FILE *stream) {
  char *p = s;
  while (size-- > 1) {
    *p = *(char *)(*stream)++;
    if (!*p) break;
    if (*p == '\n') { *(p+1) = 0; break; };
    p++;
  }
  return s;
}

char *textFile();

char inpBuf[2048] , outBuf[2048] = "DIDL\0\x01\x71";
char *dataFile, *saveFile;
char *outP, *inpP, *inpEnd;
char *fpdata[1], *fpsave[1], *fptext[1];

FILE *fopentext() { *fptext = textFile(); return fptext; }
FILE *fopendata() { *fpdata = dataFile; return fpdata; }
FILE *fopensave() { *fpsave = saveFile; return fpsave; }

// Can't use `reply_append`; IDL requires string length in advance.
int puts(const char *s) {
  while (*s) *outP++ = *s++;
  *outP++ = 10;
  return 1;
}

int putchar(int c) {
  *outP++ = c;
  return c;
}

char *gets(char *s) {
  char *p = s;
  while (inpP < inpEnd) *p++ = *inpP++;
  *p = 0;
  return s;
}

extern void go();

void grow_memory_to(int);

void init() {
  static int once = 0;
  if (once) return;
  once = 1;
  grow_memory_to(10);
  dataFile = malloc(65536);
  saveFile = malloc(65536);
}

void play() WASM_EXPORT("canister_update play");
void play() {
  init();
  int msgLen = arg_size();
  arg_copy(inpBuf, 0, msgLen);
  inpEnd = inpBuf + msgLen;
  outP = outBuf + 12;
  inpP = msgLen < 7 ? inpEnd : inpBuf + 8;  // Assume length takes 1 byte.

  go();
  int n = outP - outBuf - 12;
  outBuf[7] = (n & 255) | 128;
  outBuf[8] = ((n >> 7) & 255) | 128;
  outBuf[9] = ((n >> 14) & 255) | 128;
  outBuf[10] = ((n >> 21) & 255) | 128;
  outBuf[11] = n >> 28;

  reply_append(outBuf, outP - outBuf);
  reply();
}
