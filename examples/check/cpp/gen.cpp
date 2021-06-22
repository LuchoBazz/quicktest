#include <bits/stdc++.h>
using namespace std;

int main(int argc, char* argv[]) {
    // Generator
    // Problem: given n, give 2 numbers that multiplied give n

    int seed = stoi(string(argv[1]));
    srand(seed);

    int n = rand() % int(1e5) + 1; // [1, 1e5]
    cout << n << "\n";

    return 0;
}