/*
 * naive implementation
 */

#include <stdio.h>
#include <limits.h>
#include <string.h>

void main() {
    char out[20];

    for (int i = 1; i < INT_MAX; i++) {
        out[0] = '\0';
        if (i % 3 == 0) {
            strcat(out, "fizz");
        }
        if (i % 5 == 0) {
            strcat(out, "buzz");
        }
        if (strcmp(out, "") == 0) {
            sprintf(out, "%d", i);
        }
        printf("%s\n", out);
    }
}
