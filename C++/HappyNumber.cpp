//Write an algorithm to determine if a number n is happy.
//
//A happy number is a number defined by the following process:
//
//Starting with any positive integer, replace the number by the sum of the squares of its digits.
//Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
//Those numbers for which this process ends in 1 are happy.
//Return true if n is a happy number, and false if not.

class Solution {
public:
    bool isHappy(int n) {
        if (n == 1) return true;
        auto squareSumDigits = [](int n)
        {
            auto result = 0;
            while (n > 0)
            {
                auto temp = n % 10;
                result += temp * temp;
                n /= 10;
            }
            return result;
        };
        auto slow = squareSumDigits(n);
        if (slow == 1) return true;
        auto fast = squareSumDigits(slow);

        while (fast != slow)
        {
            if (fast == 1) return true;
            fast = squareSumDigits(squareSumDigits(fast));
            slow = squareSumDigits(slow);
        }
        return false;
    }
};
