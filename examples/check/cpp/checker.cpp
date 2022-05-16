#include <bits/stdc++.h>
using namespace std;

int main() {
    // Checker
    // Problem: given n, give 2 numbers that multiplied give n

    // read n from input
    ifstream in (".qt/input.txt");
    int n; in >> n;

    // read answer
    int x, y;
    cin >> x >> y;

    cout << (x*y == n?"YES":"NO") << endl;
    
    return 0;
}