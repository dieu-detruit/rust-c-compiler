#include <stdio.h>

int print(int x, int y, int z, int w, int u, int v, int p1, int p2)
{
    printf("Main Value: %d, %d\n", x, y);
    printf("Sub Value: %d, %d, %d, %d, %d, %d\n", z, w, u, v, p1, p2);

    return 42;
}

void print_num(int n)
{
    printf("num print: %d\n", n);
}
