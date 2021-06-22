#include <bits/stdc++.h>
using namespace std;

int main(int argc,char* argv[]) {
    // Generator for Maximum Subarray Problem

    // quicktest passes a seed as an argument
    int seed = stoi(string(argv[1]));
    srand(seed);

    int n = rand() % int(1e3) + 1;
    cout << n << "\n";

    default_random_engine generator;
    uniform_int_distribution<int> distribution(-int(1e2), int(1e2));

    for(int i = 0; i < n; i++) {
        if(i > 0)
            cout << " ";
        cout << distribution(generator);
    }
 
    return 0;
}