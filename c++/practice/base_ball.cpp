#include <iostream>
#include <string>
#include <vector>

#include <gtest/gtest.h>

using namespace std;

int calcPoints(vector <string> input) 
{
    vector<int> out{};

    int deleted = 0;

    for (string ch : input) {   
        try {
            int val = std::stoi(ch); // Throws: no conversion
            out.push_back(val);
        } catch (std::invalid_argument const& ex) {
            if (ch == "D") {
                int crt = out.back();
                out.pop_back();
                out.push_back(crt);
                out.push_back(crt * deleted);
            } else if (ch == "C") {
                deleted = out.back(); out.pop_back();
            } else if (ch == "+") {
                int crt = out.back(); out.pop_back();
                int prev = out.back();out.pop_back();
                out.push_back(prev);
                out.push_back(crt);
                out.push_back(crt + prev);
            }
        }
    }
    
    int sum = 0;
    for (int elem : out)
        sum += elem;

    return sum;
}

TEST(ParanthesisTest, BasicAssertions1) {
    vector <string> items{"5","2","C","D","+"};

    int sum = calcPoints(items);
    EXPECT_EQ(sum, 30);
}

TEST(ParanthesisTest, BasicAssertions2) {
     vector <string> items{"5","-2","4","C","D","9","+","+"};

    // Their solution is 27 , which I think is wrong!
    int sum = calcPoints(items);
    EXPECT_EQ(sum, 27);
    // EXPECT_EQ(sum, 15);
}

TEST(ParanthesisTest, BasicAssertions3) {
     vector <string> items{"1"};

    int sum = calcPoints(items);
    EXPECT_EQ(sum, 1);
}

int main(int argc, char **argv) 
{
    ::testing::InitGoogleTest(&argc, argv);

    return RUN_ALL_TESTS();
}
