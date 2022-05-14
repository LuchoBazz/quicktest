#include <bits/stdc++.h>
using namespace std;

int main() {
    // Checker
    // Problem: given n, give 2 numbers that multiplied give n

    // read n from input
    ifstream in (".qtest/input.txt");
    int n; in >> n;

    // read answer
    int x, y;
    cin >> x >> y;

    if(x*y == n) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
    
    return 0;
}