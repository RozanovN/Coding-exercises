//Return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//Return 0 when needle is an empty string.

class Solution {
    public int strStr(String haystack, String needle) {
        if (haystack.equals(needle) || needle.equals("")) {
            return 0;
        }
        for (int i = 0; i < haystack.length(); i++) {
            if (i + needle.length() > haystack.length()) {
                return -1;
            }
            if (haystack.charAt(i) == needle.charAt(0)) {
                if (haystack.substring(i, i + needle.length()).equals(needle)) {
                    return i;
                }
            }
        }
        return -1;
    }
}
