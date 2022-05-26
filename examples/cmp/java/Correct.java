import java.util.*;

public class Correct {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int[] values = new int[n];
        for(int i = 0; i < n; ++i) {
            values[i] = sc.nextInt();
        }
        int best = 0;
        for (int i = 0; i < n; i++) {
            int sum = 0;
            for (int j = i; j < n; j++) {
                sum += values[j];
                best = Math.max(best, sum);
            }
        }
        System.out.println(best);
    }
}
