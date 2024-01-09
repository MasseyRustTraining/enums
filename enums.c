#include <stdio.h>

enum color {
    RED = 1,
    GREEN = 2,
    BLUE = 4,
};

int main() {
    enum color c = GREEN;
    printf("%d\n", c);
    return 0;
}
