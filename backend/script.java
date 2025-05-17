import java.util.*;
import java.io.*;
import java.lang.*;;

public class script {

    ArrayList<LinkedList<String>> titles = new ArrayList<>();
    ArrayList<LinkedList<String>> authors = new ArrayList<>();
    ArrayList<String> synopsis = new ArrayList<>();
    ArrayList<LinkedList<Genre>> genres = new ArrayList<>();

    String filepath = "~/Documents/Bestest-website/backend/manhwa.txt";

    public void initialize() throws FileNotFoundException {
        File f = new File(filepath);

        Scanner scan = null;
        try {
            scan = new Scanner(f);
        } catch (FileNotFoundException e) {
            scan.close();
            throw new FileNotFoundException();
        }
        while (scan.hasNextLine()) {
            String temp = scan.nextLine();
            int end = temp.indexOf(" ");
            create(temp, end);
        }
    }

    public void create(String s, int start) {
        switch (s.substring(0, start)) {
            case "TITLE:":
                title(s.substring(start + 1));
            case "AUTHOR:":
                author(s.substring(start + 1));
            case "SYNOPSIS:":
                synopsis.add(s.substring(start + 1));
                return;
            case "GENRES:":
                genre(s.substring(start + 1));
            default:
                break;
        }
    }

    public void title(String s) {

    }

    public void author(String s) {

    }

    public void genre(String s) {

    }

    public static void main(String[] args) {
        try {
            initialize();
        } catch (FileNotFoundException e) {
            System.out.println("You fucked up");
            return;
        }
    }
}

class Genre {
    public String the_genre;
    public int score;
}
