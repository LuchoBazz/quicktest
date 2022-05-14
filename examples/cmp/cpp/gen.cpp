#include <bits/stdc++.h>
using namespace std;

int main(int argc, char* argv[]) {
    // Generator for Maximum Subarray Problem

    // quicktest passes a seed as an argument: $ ./main seed testcase
    int seed = stoi(string(argv[1]));
    srand(seed);

    const int N = int(1e5);
    const int Ai = int(1e5);

    int n = rand() % N + 1;
    cout << n << "\n";

    default_random_engine generator;
    uniform_int_distribution<int> distribution(-Ai, Ai);

    for(int i = 0; i < n; i++) {
        if(i > 0) cout << " ";
        cout << distribution(generator);
    }
 
    return 0;
}