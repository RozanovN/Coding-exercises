#include <iostream>
#include <vector>

// Given an integer numRows, return the first numRows of Pascal's triangle.

// In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:
// https://upload.wikimedia.org/wikipedia/commons/0/0d/PascalTriangleAnimated2.gif

// Example 1:

// Input: numRows = 5
// Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]
// Example 2:

// Input: numRows = 1
// Output: [[1]]
 

// Constraints:

// 1 <= numRows <= 30

class Solution {
public:
    std::vector<std::vector<int>> generate(int numRows) {
        std::vector<std::vector<int>> triangle{static_cast<size_t>(numRows)};
        int raw = 1;

        std::vector<int> *prev = nullptr;
        for (auto& cells : triangle)
        {
            for (int i = 0; i < raw; i++)
            {
                if (!(i==0 || i == raw - 1))
                {
                    cells.push_back(
                        (*prev)[get_index(i - 1, prev->size())]
                        + (*prev)[get_index(i, prev->size())]);
                } else {
                    cells.push_back(1);
                }
            }
            prev = &cells;
            ++raw;
        }
        return triangle;
    }

    int get_index(int i, int size)
    {
        if (i < 0 || i >= size) {
            return 0;
        } else {
            return i;
        }
    }
};

// Runtime: 0 ms, faster than 100.00% of C++ online submissions for Pascal's Triangle.
// Memory Usage: 6.4 MB, less than 90.80% of C++ online submissions for Pascal's Triangle.