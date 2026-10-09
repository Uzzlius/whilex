clang -Oz \
  -ffunction-sections -fdata-sections \
  -fno-unwind-tables -fno-asynchronous-unwind-tables \
  -fno-ident \
  -c runtime.c -o runtime.o
