// Given an integer x, return true if x is palindrome integer.

// An integer is a palindrome when it reads the same backward as forward.

// For example, 121 is a palindrome while 123 is not.
class Solution {
    public boolean isPalindrome(int x) {
        if (x < 0) {
            return false;
        }
        String z = String.valueOf(x);
        for (int i = 0; i < z.length(); i++)
            if (z.charAt(i) != z.charAt(z.length() - 1 - i)) {
                return false;
            }
        return true;  
    }
}
