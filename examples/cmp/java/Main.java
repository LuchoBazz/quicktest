import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] values = new int[n];
        for(int i = 0; i < n; ++i) {
            values[i] = sc.nextInt();
        }
        int best = 0, sum = 0;
        // i+1 was added intensively to generate an error
        for (int i = 0; i+1 < n; i++) {
            sum = Math.max(values[i], sum + values[i]);
            best = Math.max(best, sum);
        }
        System.out.println(best);
    }
}