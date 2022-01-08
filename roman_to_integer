#  Given a roman numeral, convert it to an integer.
class Solution:
    def romanToInt(self, s: str) -> int:
        roman_dict = {
            "I": 1,
            "V": 5,
            "X": 10,
            "L": 50,
            "C": 100,
            "D": 500,
            "M": 1000
        }
        result = 0
        for index in range(len(s) - 1):
            if roman_dict[s[index]] < roman_dict[s[index + 1]]:
                result -= roman_dict[s[index]]
            else:
                result += roman_dict[s[index]]
        result += roman_dict[s[-1]]
        return result
