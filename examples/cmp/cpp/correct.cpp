#include <bits/stdc++.h>
using namespace std;

int main() {
    // Maximum Subarray Problem

    int n; cin >> n;
    vector<int> values(n);
    for(int &a: values)
        cin >> a;
    
    int best = 0, sum = 0;
    
    // i+1 was added intensively to generate an error
    for (int i = 0; i < n; i++) {
        sum = max(values[i], sum + values[i]);
        best = max(best, sum);
    }
    cout << best << "\n";
    
    return 0;
}