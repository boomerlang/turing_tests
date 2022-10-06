#include <iostream>
#include <string>
#include <vector>
#include <map>

#include "gtest/gtest.h"

using namespace std;

bool valid_parant(string input)
{
    char ch;
    const char* input0 = input.c_str();
    vector <char> temp{};
    std::map<char, char> pairs;

    pairs['('] = ')';
    pairs['['] = ']';
    pairs['{'] = '}';

    do {
        ch = *input0;

        if (temp.size() == 0) {
            temp.push_back(pairs[ch]);
        } else {
            if (ch == temp.back()) {
                temp.pop_back();
            } else {
                temp.push_back(pairs[ch]);
            }
        }
    } while(*input0++);

    int sz = temp.size();
    if (sz == 1) 
        return true;
    
    return false;
}

TEST(ParanthesisTest, BasicAssertions1) {
  // Expect two strings not to be equal.
//   EXPECT_STRNE("hello", "world");
  // Expect equality.
//   EXPECT_EQ(7 * 6, 42);

    EXPECT_FALSE(valid_parant("([)]"));
}

TEST(ParanthesisTest, BasicAssertions2) {
    EXPECT_TRUE(valid_parant("{[]}"));
}

TEST(ParanthesisTest, BasicAssertions3) {
    EXPECT_TRUE(valid_parant("()[]{}"));
}

TEST(ParanthesisTest, BasicAssertions4) {
    // EXPECT_TRUE(valid_parant("()[({})]{[({})]}]"));
    EXPECT_TRUE(valid_parant("()[({})]{[({})]}"));
}


int main(int argc, char **argv) 
{
    ::testing::InitGoogleTest(&argc, argv);

    return RUN_ALL_TESTS();
}
