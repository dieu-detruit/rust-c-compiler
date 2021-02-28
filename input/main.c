test_for = 1;
result = 1;
i = 0;
if (test_for) {
    for (; i < 10; i = i + 1) {
        result = 2 * result;
    }
} else {
    while (result < 30) {
        result = 3 * result;
    }
}
return print(i, result, 11, 12, 13, 14, 15, 16);
