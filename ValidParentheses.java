import java.util.Map;
import static java.util.Map.entry;

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

// An input string is valid if:

// Open brackets must be closed by the same type of brackets.
// Open brackets must be closed in the correct order.
class Solution {
    public boolean isValid(String s) {
        Map<Character, String> mapOfBrackets = Map.ofEntries(
                entry('(', "()"),
                entry('{', "{}"),
                entry('[', "[]"),
                entry(')', "()"),
                entry('}', "{}"),
                entry(']', "[]")
        );
        for (char letter : s.toCharArray())
            s = s.replace(mapOfBrackets.get(letter), "");
        return s.length() == 0;
    }
}
