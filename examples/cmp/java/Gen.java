import java.util.Random;
import java.io.PrintWriter;

public class Gen {
    public static void main(String[] args) {
        Random rand = new Random();
        PrintWriter out = new PrintWriter(System.out);
        
        final int N  = 1000;
        final int mn = -100000;
        final int mx = 100000;

        int n = rand.nextInt(N);

        out.println(n);

        for(int i = 0; i < n; i++) {
            if(i > 0)
                out.print(" ");
            int ai = (int)Math.floor(Math.random()*(mx-mn+1)+mn);
            out.print(ai);
        }
        out.flush();
    }
}
