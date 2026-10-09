
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <inttypes.h>
#include <limits.h>

typedef __uint128_t u128;

extern u128 _main();

static size_t g_argc;
static u128 *g_argv;

static u128 parse(const char *s) {
    if (s == NULL || *s == '\0' || *s == '-') {
        fprintf(stderr, "Invalid unsigned integer argument\n");
        exit(EXIT_FAILURE);
    }

    u128 value = 0;

    for (; *s != '\0'; ++s) {
        if (*s < '0' || *s > '9') {
            fprintf(stderr, "Invalid integer argument\n");
            exit(EXIT_FAILURE);
        }

        unsigned digit = (unsigned)(*s - '0');
        u128 max = ~(u128)0;

        if (value > (max - digit) / 10) {
            fprintf(stderr, "Integer argument out of range\n");
            exit(EXIT_FAILURE);
        }

        value = value * 10 + digit;
    }

    return value;
}

static void print_u128(u128 value) {
    char buffer[40];
    size_t i = sizeof(buffer);

    do {
        buffer[--i] = (char)('0' + value % 10);
        value /= 10;
    } while (value != 0);

    fwrite(&buffer[i], 1, sizeof(buffer) - i, stdout);
    putchar('\n');
}

u128 _load_arg(uint32_t idx) {
    if (idx >= (uint32_t)g_argc) {
        return 0;
    }

    return g_argv[idx];
}

int main(int argc, char **argv) {
    g_argc = argc-1;

    g_argv = calloc(g_argc, sizeof(*g_argv));
    if (g_argv == NULL && g_argc != 0) {
        perror("calloc");
        return EXIT_FAILURE;
    }

    for (int i = 0; i < g_argc; ++i) {
        g_argv[i] = parse(argv[i+1]);
    }

    u128 result = _main();
    print_u128(result);

    free(g_argv);
    return 0;
}
