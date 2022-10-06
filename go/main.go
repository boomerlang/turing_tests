package main

import (
	"fmt"
	"boomerlang.eu/turing/v2/test_lib"
)

func main () {

	input := []string{"5","2","C","D","+"}

	sum := test_lib.CalcPoints(input)

	fmt.Println(sum)

	input = []string{"5","-2","4","C","D","9","+","+"}

	sum = test_lib.CalcPoints(input)

	fmt.Println(sum)

	input1 := "([)]"
	rez := test_lib.ValidParant(input1)
	fmt.Println(rez)

	input1 = "{[]}"
	rez = test_lib.ValidParant(input1)
	fmt.Println(rez)

	input1 = "()[]{}"
	rez = test_lib.ValidParant(input1)
	fmt.Println(rez)

	input1 = "()[({})]{[({})]}"
	rez = test_lib.ValidParant(input1)
	fmt.Println(rez)
}
