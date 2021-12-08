/*
 * A slightly simpler, faster implementation, with no allocation
 */

#include <stdio.h>
#include <limits.h>

void main() {
    for (int i = 1; i < INT_MAX; i++) {
        if (i % 15 == 0) {
            printf("fizzbuzz\n");
        } else if (i % 3 == 0) {
            printf("fizz\n");
        } else if (i % 5 == 0) {
            printf("buzz\n");
        }
        else {
            printf("%d\n", i);
        }
    }
}
