#include <stdio.h>
#include <string.h>
#include <stdlib.h>

int main(int argc, char *argv[]) {
    int seed;
    sscanf(argv[1], "%d", &seed);

    srand(seed); 
    int r = rand();

    #define N  1000
    #define mx 100000
    #define mn (-100000)

    int n = rand() % N + 1;
    printf("%d\n", n);

    for(int i = 0; i < n; i++) {
        if(i > 0) printf(" ");
        printf("%d",  mn + rand() % ( mx - mn + 1 ));
    }
    fflush(stdout);

    return 0;
}