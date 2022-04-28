#include <bits/stdc++.h>
using namespace std;

int main() {
    // Maximum Subarray Problem
    // 1 <= n <= 1e5
    // -1e5 <= a_i <= 1e5
    
    int n; cin >> n;
    vector<int> values(n);
    for(int &a: values)
        cin >> a;
    
    int best = 0;
    for (int i = 0; i < n; i++) {
        int sum = 0;
        for (int j = i; j < n; j++) {
            sum += values[j];
            best = max(best, sum);
        }
    }
    cout << best << "\n";
    return 0;
}