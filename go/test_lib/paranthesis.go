package test_lib

import (
	// "fmt"
)

func ValidParant(input string) bool {

	temp := []rune{}
	pairs := map[rune]rune{'(': ')', '[': ']', '{': '}'}

	for _, ch := range input {
		if len(temp) == 0 {
			temp = append(temp, pairs[ch])
		} else {
			last := temp[len(temp) - 1]
			if ch == last {
				temp = temp[0:len(temp) - 1]
			} else {
				temp = append(temp, pairs[ch])
			}
		}
	}

	sz := len(temp);

	return sz == 0
}
