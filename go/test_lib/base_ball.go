package test_lib

import (
	// "fmt"
	"strconv"
)

func CalcPoints(input []string) int {

	var out []int = []int{}
	deleted := 0

	for _, ch := range input {
		val, err := strconv.Atoi(string(ch))

		if err == nil {
			out = append(out, val)
		} else {
			if ch == "D" {
				crt := out[len(out) - 1]
				out = out[0:len(out) - 1]
				out = append(out, crt, crt * deleted)
			}
			if ch == "C" {
				deleted = out[len(out) - 1]
				out = out[0:len(out) - 1]
			}
			if ch == "+" {
				x := out[len(out)-2:len(out)]
				out = out[0:len(out) - 2]
				out = append(out, x[0], x[1], x[0] + x[1])
			}
		}
	}
	
	sum := 0
	for _, elem  := range out {
		sum += elem
	}

	return sum
}
