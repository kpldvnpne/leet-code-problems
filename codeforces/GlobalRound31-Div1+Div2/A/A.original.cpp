
#include <stdio.h>

int gcd(int a, int b) {
    if (a % b == 0) {
        return b;
    } else {
        return gcd(b, a % b);
    }
}

int solution(int l, int a, int b) {
    // b = b % l;
    int d = gcd(b, l);
    return l - d + (a % d);
}

int main(int argc, char **argv) {
    int t;
    scanf("%d", &t);

    for (int i = 0; i < t; ++i) {
        int l, a, b;
        scanf("%d %d %d", &l, &a, &b);

        int answer = solution(l, a, b);
        printf("%d\n", answer);
    }

    return 0;
}