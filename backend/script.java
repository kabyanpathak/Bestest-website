import java.util.*;

public class script {

    ArrayList<LinkedList<String>> titles = new ArrayList<>();
    ArrayList<LinkedList<String>> authors = new ArrayList<>();
    ArrayList<String> synopsis = new ArrayList<>();
    ArrayList<Genre> genres = new ArrayList<>();

    String filepath = "~/Documents/Bestest-website/backend/manhwa.txt";

    public void initialize() {

    }

    public static void main(String[] args) {

    }
}

class Genre {
    public String the_genre;
    public int score;
}
