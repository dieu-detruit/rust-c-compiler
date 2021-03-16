int fibonacci(int n)
{
    if (n == 0) {
        return 1;
    }
    if (n == 1) {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

int main()
{
    int i;
    int j;
    i = 0;
    j = 3;
    for (; i < 10; i = i + 1) {
        print_num(fibonacci(i));
    }

    return 0;
}
